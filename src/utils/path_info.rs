use std::path::PathBuf;

#[derive(Clone)]
pub struct PathInfo {
    pub data_quotes_path: PathBuf,
    pub data_trades_path: PathBuf,
}
