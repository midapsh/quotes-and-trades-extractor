use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use crate::utils::instrument_info::InstrumentInfo;
use crate::utils::path_info::PathInfo;

#[async_trait]
pub trait Worker: Send {
    fn new(
        cancellation_token: CancellationToken,
        instrument_info: InstrumentInfo,
        path_info: PathInfo,
    ) -> Self;
    async fn execute(&mut self) -> std::io::Result<()>;
    fn stop(&self);
}
