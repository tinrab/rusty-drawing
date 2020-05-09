use crate::message::{ClientEgressMessage, ClientIngressMessage, IngressMessage};
use futures::prelude::stream::*;
use futures::prelude::*;
use log::info;
use tokio::net::TcpStream;
use tokio::sync::{broadcast, mpsc};
use tokio_tungstenite::{tungstenite, WebSocketStream};
use uuid::Uuid;

pub struct Client {
    id: Uuid,
}

impl Client {
    pub fn new() -> Self {
        Client { id: Uuid::new_v4() }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub async fn read_ingress(
        &self,
        stream: SplitStream<WebSocketStream<TcpStream>>,
        sender: mpsc::UnboundedSender<ClientIngressMessage>,
    ) {
        let client_id = self.id.clone();
        stream
            .try_for_each(|message: tungstenite::Message| {
                if !message.is_text() {
                    return future::ok(());
                }
                let data = message.to_text().unwrap().trim();
                let ingress_message = match serde_json::from_str::<IngressMessage>(data) {
                    Ok(ingress_message) => ingress_message,
                    Err(err) => {
                        info!("Invalid JSON message received: {}", err);
                        return future::err(tungstenite::error::Error::Http(
                            tungstenite::http::StatusCode::BAD_REQUEST,
                        ));
                    }
                };
                sender
                    .send(ClientIngressMessage::new(client_id, ingress_message))
                    .unwrap();
                future::ok(())
            })
            .await;
    }

    pub async fn write_egress(
        &self,
        sink: &mut SplitSink<WebSocketStream<TcpStream>, tungstenite::Message>,
        receiver: &mut broadcast::Receiver<ClientEgressMessage>,
    ) {
        let client_id = self.id.clone();
        while let Ok(egress_message) = receiver.recv().await {
            if egress_message.source_client_id() == Some(client_id) {
                continue;
            }
            let data = serde_json::to_string(egress_message.message()).unwrap();
            sink.send(tungstenite::Message::text(data)).await;
        }
    }
}
