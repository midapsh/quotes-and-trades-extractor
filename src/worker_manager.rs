use std::collections::{BTreeMap, BTreeSet};

// use async_trait::async_trait;
use tokio::time::{sleep_until, Duration, Instant};
use tokio_util::sync::CancellationToken;

pub struct WorkerManager {
    update_time: u64,
    venue: String,
    instruments: BTreeSet<String>,
    workers: BTreeSet<Worker>,
    workers_tokens: BTreeMap<String, CancellationToken>,
}

pub struct Worker {
    cancellation_token: CancellationToken,
}

impl Worker {
    fn new(cancellation_token: CancellationToken) -> Self {
        Self { cancellation_token }
    }

    async fn execute(self) {}
    // while !self.cancellation_token.is_cancelled() {
    //     // TODO(hspadim): if <weird error>
    //     // self.cancellation_token.cancel();
    // }
}

// #[async_trait]
// pub trait Worker {
//     fn new(cancellation_token: CancellationToken);

//     async fn execute(self);
//     // while !self.cancellation_token.is_cancelled() {
//     //     // TODO(hspadim): if <weird error>
//     //     // self.cancellation_token.cancel();
//     // }
// }

impl WorkerManager {
    fn new(update_time: u64, venue: String) -> Self {
        Self {
            update_time,
            venue,
            instruments: BTreeSet::new(),
            workers: BTreeSet::new(),
            workers_tokens: BTreeMap::new(),
        }
    }

    async fn get_instruments_from_db(&self) -> BTreeSet<String> {
        unimplemented!()
    }

    async fn refresh_instruments(&mut self) {
        let most_recent_instruments = self.get_instruments_from_db().await;

        let instruments_to_be_added: BTreeSet<String> = most_recent_instruments
            .difference(&self.instruments)
            .cloned()
            .collect();
        let mut instruments_to_be_removed: BTreeSet<String> = self
            .instruments
            .difference(&most_recent_instruments)
            .cloned()
            .collect();

        if instruments_to_be_added.is_empty() & instruments_to_be_removed.is_empty() {
            return;
        }

        // NOTE(hspadim): Check if there is a worker that is dead
        for (instrument, cancellation_token) in self.workers_tokens.iter() {
            if cancellation_token.is_cancelled() {
                instruments_to_be_removed.insert(instrument.to_string());
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
        for instrument in instruments_to_be_added.iter() {
            let cancellation_token = CancellationToken::new();
            let maybe_cancellation_token = self
                .workers_tokens
                .insert(instrument.to_string(), cancellation_token);
            if let Some(cancellation_token) = maybe_cancellation_token {
                let worker = Worker::new(cancellation_token);
                tokio::spawn(async { worker.execute().await });
                self.workers.insert(worker);
            }
        }

        self.instruments = most_recent_instruments;
    }

    pub async fn execute(&mut self) {
        loop {
            let deadline = Instant::now() + Duration::from_secs(self.update_time);
            self.refresh_instruments().await;
            sleep_until(deadline).await;
        }
    }
}
