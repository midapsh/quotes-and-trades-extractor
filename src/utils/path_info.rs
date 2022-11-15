use std::path::PathBuf;

pub struct PathInfo<'a> {
    data_quotes_path: &'a PathBuf,
    data_trades_path: &'a PathBuf,
}
