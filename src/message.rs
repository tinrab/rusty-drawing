use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct IngressMessage {
    pub client_id: Uuid,
    pub message: Message,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum EgressMessage {
    Broadcast {
        ignored_client_id: Option<Uuid>,
        message: Message,
    },
    Direct {
        client_id: Uuid,
        message: Message,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
#[non_exhaustive]
pub enum Message {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "disconnected")]
    Disconnected,
    #[serde(rename = "user-joined")]
    UserJoined(UserJoinedMessage),
    #[serde(rename = "user-left")]
    UserLeft(UserLeftMessage),
    #[serde(rename = "stroke")]
    Stroke(StrokeMessage),
    #[serde(rename = "clear")]
    Clear(ClearMessage),
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ConnectedMessage {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserJoinedMessage {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLeftMessage {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StrokeMessage {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearMessage {}
