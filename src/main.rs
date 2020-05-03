use rusty_drawing::error::{RustyError, RustyResult};
use rusty_drawing::server::Server;
use std::error::Error;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> RustyResult<()> {
    Server::run(8080).await
}
