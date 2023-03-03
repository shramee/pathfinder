// Types used for web socket subscription events
use crate::reply::Block;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SubscriptionSyncEvent {
    pub block: Box<Block>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubscriptionNewHeadEvent {
    pub block: Box<Block>,
}

/// Events and queries emitted by L2 sync process.
#[derive(Debug, Clone, Serialize)]
pub enum SubscriptionEvent {
    Sync(SubscriptionSyncEvent),
    NewHead(SubscriptionNewHeadEvent),
}
