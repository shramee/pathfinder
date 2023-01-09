use jsonrpsee::core::server::rpc_module::Methods;
use tokio::sync::broadcast;

use crate::context::RpcContext;

pub mod subscription;

/// Registers all methods for the v0.2 RPC API
pub fn register_subscriptions(
    context: RpcContext,
    tx_ws_l2: broadcast::Sender<std::string::String>,
) -> anyhow::Result<Methods> {
    let methods = crate::module::Module::new(context)
        .register_subscription(
            "starknet_subscribe_newHeads",
            "s_newHeads",
            "starknet_unsubscribe_newHeads",
            subscription::subscribe_new_heads::subscribe_new_heads,
            tx_ws_l2,
        )?
        .build();

    Ok(methods)
}
