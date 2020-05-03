use crate::error::{RustyError, RustyResult};
use futures_util::{StreamExt, TryStreamExt};
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};

pub struct Server {}

impl Server {
    pub async fn run(port: u16) -> RustyResult<()> {
        let server = Arc::new(Mutex::new(Server {}));

        let mut listener = TcpListener::bind(format!("127.0.0.1:{}", port))
            .await
            .map_err(|e| RustyError::System(e.to_string()))?;
        while let Ok((stream, address)) = listener.accept().await {
            tokio::spawn(Self::handle_connection(stream));
        }
        Ok(())
    }

    async fn handle_connection(stream: TcpStream) -> RustyResult<()> {
        let ws_stream = tokio_tungstenite::accept_async(stream)
            .await
            .map_err(|e| RustyError::WebSocket(e.to_string()))?;
        let (sender, receiver) = mpsc::unbounded();
        let (outgoing, incoming) = ws_stream.split();

        Ok(())
    }
}
