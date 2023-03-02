use crate::context::RpcContext;
use jsonrpsee::core::error::SubscriptionClosed;
use jsonrpsee::core::server::rpc_module::PendingSubscription;
use starknet_gateway_types::websocket::SubscriptionEvent;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

pub fn subscribe_new_heads(
    _context: RpcContext,
    pending: PendingSubscription,
    event_txs: &broadcast::Sender<SubscriptionEvent>,
) {
    let event_txs = BroadcastStream::new(event_txs.subscribe());
    let mut sink = match pending.accept() {
        Some(sink) => sink,
        _ => return,
    };

    tokio::spawn(async move {
        match sink.pipe_from_try_stream(event_txs).await {
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
