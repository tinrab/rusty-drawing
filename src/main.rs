use rusty_drawing::error::{RustyError, RustyResult};
use rusty_drawing::server::Server;
use std::error::Error;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> RustyResult<()> {
    env_logger::init();

    let server = Server::new(8080);
    server.run().await
}
