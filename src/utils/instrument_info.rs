#[derive(Clone, Debug)]
pub struct InstrumentInfo {
    pub instrument: String,
    pub instrument_parsed: String,
}

impl InstrumentInfo {
    pub fn new(instrument: String, instrument_parsed: String) -> Self {
        Self {
            instrument,
            instrument_parsed,
        }
    }
}

impl PartialOrd for InstrumentInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.instrument.cmp(&other.instrument))
    }
}

impl PartialEq for InstrumentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.instrument == other.instrument
    }
}

impl Ord for InstrumentInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.instrument.cmp(&other.instrument)
    }
}

impl Eq for InstrumentInfo {}
