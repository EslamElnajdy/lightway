pub(crate) mod tun;

pub(crate) use tun::Tun;

use crate::connection::ConnectionState;
use async_trait::async_trait;
use lightway_core::{IOCallbackResult, InsideIOSendCallback, InsideIOSendCallbackArg};
use std::sync::Arc;

#[async_trait]
pub trait InsideIORecv: Sync + Send {
    async fn recv_buf(&self) -> IOCallbackResult<bytes::BytesMut>;

    fn into_io_send_callback(self: Arc<Self>) -> InsideIOSendCallbackArg<ConnectionState>;
}

/// Trait for InsideIO
pub trait InsideIO: InsideIORecv + InsideIOSendCallback<ConnectionState> {}
