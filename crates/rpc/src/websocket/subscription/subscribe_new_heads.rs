use crate::context::RpcContext;
use jsonrpsee::core::error::SubscriptionClosed;
use jsonrpsee::core::server::rpc_module::PendingSubscription;
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

pub fn subscribe_new_heads(
    _context: RpcContext,
    pending: PendingSubscription,
    tx_ws_l2: broadcast::Sender<std::string::String>,
) {
    let tx_ws_l2 = BroadcastStream::new(tx_ws_l2.subscribe());
    let mut sink = match pending.accept() {
        Some(sink) => sink,
        _ => return,
    };

    tokio::spawn(async move {
        match sink.pipe_from_try_stream(tx_ws_l2).await {
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
