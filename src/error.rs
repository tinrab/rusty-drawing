use serde::export::Formatter;
use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum RustyError {
    System(String),
    WebSocket(String),
}

impl Error for RustyError {}

impl Display for RustyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RustyError::System(err) => write!(f, "system error: {}", err),
            RustyError::WebSocket(err) => write!(f, "websocket error: {}", err),
        }
    }
}

pub type RustyResult<T> = Result<T, RustyError>;
