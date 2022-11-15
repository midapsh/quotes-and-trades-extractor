use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use crate::utils::instrument_info::InstrumentInfo;
use crate::utils::path_info::PathInfo;

#[async_trait]
pub trait Worker<'a> {
    fn new(
        cancelation_token: CancellationToken,
        instrument_info: InstrumentInfo<'a>,
        path_info: PathInfo<'a>,
    ) -> Self;
    pub async fn execute(self);
}
