use crate::canvas::path::Path;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ClientIngressMessage {
    client_id: Uuid,
    message: IngressMessage,
}

impl ClientIngressMessage {
    pub fn new(client_id: Uuid, message: IngressMessage) -> Self {
        ClientIngressMessage { client_id, message }
    }

    pub fn client_id(&self) -> &Uuid {
        &self.client_id
    }

    pub fn message(&self) -> &IngressMessage {
        &self.message
    }
}

#[derive(Debug, Clone)]
pub struct ClientEgressMessage {
    source_client_id: Option<Uuid>,
    message: EgressMessage,
}

impl ClientEgressMessage {
    pub fn new(source_client_id: Option<Uuid>, message: EgressMessage) -> Self {
        ClientEgressMessage {
            source_client_id,
            message,
        }
    }

    pub fn source_client_id(&self) -> Option<Uuid> {
        self.source_client_id
    }

    pub fn message(&self) -> &EgressMessage {
        &self.message
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum IngressMessage {
    #[serde(rename = "stroke")]
    Stroke(Path),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum EgressMessage {
    #[serde(rename = "alive")]
    Alive,
    #[serde(rename = "stroke")]
    Stroke(Path),
}

// #[derive(Debug, Clone)]
// pub struct IngressMessage {
//     pub client_id: Uuid,
//     pub message: Message,
// }
//
// #[derive(Debug, Clone)]
// #[non_exhaustive]
// pub enum EgressMessage {
//     Broadcast {
//         ignored_client_id: Option<Uuid>,
//         message: Message,
//     },
//     Direct {
//         client_id: Uuid,
//         message: Message,
//     },
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(tag = "type", content = "payload")]
// #[non_exhaustive]
// pub enum Message {
//     #[serde(rename = "connected")]
//     Connected,
//     #[serde(rename = "disconnected")]
//     Disconnected,
//     #[serde(rename = "user-joined")]
//     UserJoined(UserJoinedMessage),
//     #[serde(rename = "user-left")]
//     UserLeft(UserLeftMessage),
//     #[serde(rename = "stroke")]
//     Stroke(StrokeMessage),
//     #[serde(rename = "clear")]
//     Clear(ClearMessage),
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ConnectedMessage {}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct UserJoinedMessage {}
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct UserLeftMessage {}
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct StrokeMessage {}
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct ClearMessage {}
