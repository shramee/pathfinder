use jsonrpsee::core::server::rpc_module::Methods;
use starknet_gateway_types::websocket::SubscriptionEvent;
use std::collections::HashMap;
use tokio::sync::broadcast;

use crate::context::RpcContext;

pub mod subscription;

/// Registers all methods for the v0.2 RPC API
pub fn register_subscriptions(
    context: RpcContext,
    event_txs: &mut HashMap<String, broadcast::Sender<SubscriptionEvent>>,
) -> anyhow::Result<Methods> {
    let methods = crate::module::Module::new(context)
        .register_subscription(
            "starknet_subscribe_newHeads",
            "s_newHeads",
            "starknet_unsubscribe_newHeads",
            subscription::subscribe_new_heads::subscribe_new_heads,
            event_txs,
        )?
        .register_subscription(
            "starknet_subscribe_sync",
            "s_sync",
            "starknet_unsubscribe_sync",
            subscription::subscribe_sync::subscribe_sync,
            event_txs,
        )?
        .build();

    Ok(methods)
}
