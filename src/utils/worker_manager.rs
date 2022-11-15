use std::collections::{BTreeMap, BTreeSet};
use std::fs;

// use async_trait::async_trait;
use tokio::time::{interval_at, Duration, Instant};
use tokio_util::sync::CancellationToken;

use crate::configs::configuration::get_configuration;
use crate::utils::instrument_info::InstrumentInfo;
use crate::utils::path_info::PathInfo;
use crate::workers::base_worker::Worker;

pub struct WorkerManager<TWorker: Worker> {
    // NOTE(hspadim): Init vars
    update_time: u64,
    _venue: String,
    path_info: PathInfo,
    // NOTE(hspadim): Internal data
    instruments: BTreeSet<InstrumentInfo>,
    // workers: BTreeMap<InstrumentInfo, TWorker>,
    // workers: BTreeMap<InstrumentInfo, std::sync::Arc<TWorker>>,
    workers_tokens: BTreeMap<InstrumentInfo, CancellationToken>,
    _phantom_worker: std::marker::PhantomData<TWorker>,
}

impl<TWorker: Worker> WorkerManager<TWorker> {
    pub fn new(update_time: u64, venue: String) -> Self {
        // TODO(hspadim): put this into a function to create the path
        let settings = get_configuration().unwrap();

        let data_quotes_path = settings.data_quotes_path.join(&venue);
        let data_trades_path = settings.data_trades_path.join(&venue);

        fs::create_dir_all(&data_quotes_path)
            .expect(format!("Couldn't create '{}' quotes file", &venue).as_str());
        fs::create_dir_all(&data_trades_path)
            .expect(format!("Couldn't create '{}' trades file", &venue).as_str());
        let path_info = PathInfo {
            data_quotes_path,
            data_trades_path,
        };
        //
        Self {
            // NOTE(hspadim): Init vars
            update_time,
            _venue: venue,
            path_info,
            // NOTE(hspadim): Internal data
            instruments: BTreeSet::new(),
            workers_tokens: BTreeMap::new(),
            _phantom_worker: std::marker::PhantomData,
        }
    }

    async fn get_instruments_from_db(&mut self) -> BTreeSet<InstrumentInfo> {
        let list_instruments: BTreeSet<InstrumentInfo> = [
            // (".BBCH", ".BBCH"),
            // (".BUSDT", ".BUSDT"),
            // (".BLINKT", ".BLINKT"),
            // (".BADAT", ".BADAT"),
            // (".BDOTT", ".BDOTT"),
            // (".BDOGET", ".BDOGET"),
            // (".BSOLT", ".BSOLT"),
            // (".BSUSHIT", ".BSUSHIT"),
            // (".BDOGE", ".BDOGE"),
            // (".BDOT", ".BDOT"),
            // (".BLINK", ".BLINK"),
            // (".BSOL", ".BSOL"),
            // (".BXRPT", ".BXRPT"),
            // (".BBCHT", ".BBCHT"),
            // (".BSHIBT", ".BSHIBT"),
            ("XRPUSD", "XRPUSD"),
            ("BCHUSD", "BCHUSD"),
            ("DOGEUSD", "DOGEUSD"),
            ("BNBUSD", "BNBUSD"),
            ("LINKUSD", "LINKUSD"),
            ("SOLUSD", "SOLUSD"),
            ("LINKUSDT", "LINKUSDT"),
            ("DOGEUSDT", "DOGEUSDT"),
            ("DOTUSDT", "DOTUSDT"),
            ("ADAUSDT", "ADAUSDT"),
            ("BNBUSDT", "BNBUSDT"),
            ("SOLUSDT", "SOLUSDT"),
            ("ADAUSD", "ADAUSD"),
            ("XRPUSDT", "XRPUSDT"),
            ("BCHUSDT", "BCHUSDT"),
            ("DOTUSD", "DOTUSD"),
            ("AVAXUSD", "AVAXUSD"),
            ("SHIBUSDT", "SHIBUSDT"),
            ("AVAXUSDT", "AVAXUSDT"),
            ("UNI_USDT", "UNI_USDT"),
            ("LINK_USDT", "LINK_USDT"),
            // (".BXBT", ".BXBT"),
            // (".BVOL", ".BVOL"),
            // (".BVOL24H", ".BVOL24H"),
            // (".BVOL7D", ".BVOL7D"),
            // (".BETH", ".BETH"),
            // (".BETHT", ".BETHT"),
            // (".BLTC", ".BLTC"),
            // (".BLTCT", ".BLTCT"),
            ("XBTUSD", "XBTUSD"),
            ("XBTUSDT", "XBTUSDT"),
            ("ETHUSD", "ETHUSD"),
            ("ETHUSDT", "ETHUSDT"),
            ("ETH_USDT", "ETH_USDT"),
            ("LTCUSD", "LTCUSD"),
            ("LTCUSDT", "LTCUSDT"),
            // ("ADAU22", "ADAU22"),
            // ("XRPU22", "XRPU22"),
            // ("ETHU22", "ETHU22"),
        ]
        .iter()
        .map(|&(instrument, instrument_parsed)| {
            InstrumentInfo::new(instrument.to_string(), instrument_parsed.to_string())
        })
        .collect();
        list_instruments
    }

    async fn refresh_instruments(&mut self) {
        let most_recent_instruments: BTreeSet<InstrumentInfo> =
            self.get_instruments_from_db().await;

        let mut instruments_to_be_added: BTreeSet<InstrumentInfo> = most_recent_instruments
            .difference(&self.instruments)
            .cloned()
            .collect();
        let instruments_to_be_removed: BTreeSet<InstrumentInfo> = self
            .instruments
            .difference(&most_recent_instruments)
            .cloned()
            .collect();

        if instruments_to_be_added.is_empty() & instruments_to_be_removed.is_empty() {
            return;
        }

        // NOTE(hspadim): Check if there is a worker that is dead
        for (instrument_info, cancellation_token) in self.workers_tokens.iter() {
            if cancellation_token.is_cancelled() {
                // NOTE(hspadim): If the instrument shouldn't be removed,
                // try to init it again
                if !instruments_to_be_removed.contains(&instrument_info) {
                    instruments_to_be_added.insert(instrument_info.clone());
                }
            }
        }

        // NOTE(hspadim): Remove instruments that don't exist anymore in the database
        for instrument in instruments_to_be_removed.iter() {
            let worker_token = &self.workers_tokens[instrument];
            if !worker_token.is_cancelled() {
                self.workers_tokens[instrument].cancel();
            }
            self.workers_tokens.remove(instrument);
        }

        // NOTE(hspadim): Add new instruments
        for instrument_info in instruments_to_be_added.into_iter() {
            let cancellation_token = CancellationToken::new();
            self.workers_tokens
                .insert(instrument_info.clone(), cancellation_token.clone());

            let path_info = self.path_info.clone();

            let cloned_cancellation_token = cancellation_token.clone();
            tokio::spawn(async move {
                let mut worker =
                    TWorker::new(cloned_cancellation_token, instrument_info, path_info);
                match worker.execute().await {
                    _ => {
                        worker.stop();
                    }
                }
            });
        }

        self.instruments = most_recent_instruments;
    }

    pub async fn execute(&mut self) {
        let deadline = Instant::now() + Duration::from_secs(self.update_time);
        let mut interval = interval_at(deadline, Duration::from_secs(self.update_time));
        loop {
            self.refresh_instruments().await;
            interval.tick().await;
        }
    }
}
