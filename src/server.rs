use crate::canvas::Canvas;
use crate::client::Client;
use crate::error::{RustyError, RustyResult};
use crate::message::{ClientEgressMessage, ClientIngressMessage, EgressMessage, IngressMessage};
use futures::prelude::stream::*;
use log::info;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc, Mutex};
use tokio::time;

const ALIVE_INTERVAL: Duration = Duration::from_secs(1);

pub struct Server {
    port: u16,
    canvas: Arc<Mutex<Canvas>>,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server {
            port,
            canvas: Default::default(),
        }
    }

    pub async fn run(&self) -> RustyResult<()> {
        let (broadcast_sender, _broadcast_receiver) = broadcast::channel::<ClientEgressMessage>(16);
        let (ingress_sender, ingress_receiver) = mpsc::unbounded_channel::<ClientIngressMessage>();

        tokio::spawn(Self::process(self.canvas.clone(), ingress_receiver));
        tokio::spawn(Self::broadcast_alive(broadcast_sender.clone()));

        let mut listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))
            .await
            .map_err(|e| RustyError::System(e.to_string()))?;
        info!("Running on port {}...", self.port);

        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(Self::handle_connection(
                stream,
                ingress_sender.clone(),
                broadcast_sender.subscribe(),
            ));
        }

        Ok(())
    }

    async fn process(
        canvas: Arc<Mutex<Canvas>>,
        receiver: mpsc::UnboundedReceiver<ClientIngressMessage>,
    ) {
        let mut receiver = receiver;
        while let Some(message) = receiver.recv().await {
            match message.message() {
                IngressMessage::Stroke(path) => {
                    println!("path: {:?}", path);
                    canvas.lock().await.add_path(path.clone());
                }
            }
        }
    }

    async fn broadcast_alive(broadcast_sender: broadcast::Sender<ClientEgressMessage>) {
        let mut interval = time::interval(ALIVE_INTERVAL);
        let mut alive = interval.next();

        loop {
            alive.await;
            alive = interval.next();
            broadcast_sender
                .send(ClientEgressMessage::new(None, EgressMessage::Alive))
                .unwrap();
        }
    }

    async fn handle_connection(
        stream: TcpStream,
        ingress_sender: mpsc::UnboundedSender<ClientIngressMessage>,
        egress_receiver: broadcast::Receiver<ClientEgressMessage>,
    ) -> RustyResult<()> {
        let ws_stream = tokio_tungstenite::accept_async(stream)
            .await
            .map_err(|e| RustyError::WebSocket(e.to_string()))?;
        let (mut outgoing, incoming) = ws_stream.split();
        let mut egress_receiver = egress_receiver;

        let client = Client::new();
        info!("Client {} connected", client.id());

        tokio::select! {
            _ = client.read_ingress(incoming, ingress_sender) => {},
            _ = client.write_egress(&mut outgoing, &mut egress_receiver) => {}
        }

        info!("Client {} disconnected", client.id());
        Ok(())
    }
}
