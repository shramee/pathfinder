use crate::context::RpcContext;
use jsonrpsee::core::error::SubscriptionClosed;
use jsonrpsee::core::server::rpc_module::PendingSubscription;
use starknet_gateway_types::websocket::WebsocketEventSync;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

pub fn subscribe_sync(
    _context: RpcContext,
    pending: PendingSubscription,
    ws_sync_tx: &broadcast::Sender<WebsocketEventSync>,
) {
    let ws_sync_tx = BroadcastStream::new(ws_sync_tx.subscribe());
    let mut sink = match pending.accept() {
        Some(sink) => sink,
        _ => return,
    };

    tokio::spawn(async move {
        match sink.pipe_from_try_stream(ws_sync_tx).await {
            SubscriptionClosed::Success => {
                sink.close(SubscriptionClosed::Success);
            }
            SubscriptionClosed::RemotePeerAborted => (),
            SubscriptionClosed::Failed(err) => {
                sink.close(err);
            }
        };
    });
}
