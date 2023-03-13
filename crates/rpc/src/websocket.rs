use jsonrpsee::core::server::rpc_module::Methods;
use starknet_gateway_types::websocket::WebsocketSenders;

use crate::context::RpcContext;

pub mod subscription;

/// Registers all methods for the v0.2 RPC API
pub fn register_subscriptions(
    context: RpcContext,
    ws_broadcast_txs: WebsocketSenders,
) -> anyhow::Result<Methods> {
    let methods = crate::module::Module::new(context)
        .register_subscription(
            "starknet_subscribe_newHeads",
            "s_newHeads",
            "starknet_unsubscribe_newHeads",
            subscription::subscribe_new_heads::subscribe_new_heads,
            ws_broadcast_txs.new_head.clone(),
        )?
        .register_subscription(
            "starknet_subscribe_sync",
            "s_sync",
            "starknet_unsubscribe_sync",
            subscription::subscribe_sync::subscribe_sync,
            ws_broadcast_txs.sync.clone(),
        )?
        .build();

    Ok(methods)
}
