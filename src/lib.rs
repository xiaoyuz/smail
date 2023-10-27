pub mod config;
pub mod db;
pub mod server;
pub mod state_machine;
pub mod utils;
pub mod web;

use config::log_file;
use config::log_level;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use slog::Drain;
use std::fs::OpenOptions;
use thiserror::Error;

lazy_static! {
    pub static ref LOGGER: slog::Logger = slog::Logger::root(
        slog_term::FullFormat::new(slog_term::PlainSyncDecorator::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(log_file())
                .unwrap()
        ))
        .use_custom_timestamp(utils::timestamp_local)
        .build()
        .filter_level(slog::Level::from_usize(log_level()).unwrap())
        .fuse(),
        slog::o!()
    );
}

/// Error returned by most functions.
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// A specialized `Result` type for mapuche operations.
///
/// This is defined as a convenience.
pub type Result<T> = anyhow::Result<T, Error>;

#[derive(Error, Debug)]
pub enum SmailError {
    #[error("{0}")]
    String(&'static str),
    #[error("{0}")]
    Owned(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum State {
    Fresh,
    Greeted,
    ReceivingRcpt(Mail),
    ReceivingData(Mail),
    Received(Mail),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mail {
    pub id: String,
    pub from: String,
    pub to: Vec<String>,
    pub data: String,
    pub ts: i64,
}

impl From<&Mail> for String {
    fn from(value: &Mail) -> Self {
        serde_json::to_string(value).unwrap()
    }
}

impl From<&str> for Mail {
    fn from(value: &str) -> Self {
        serde_json::from_str(value).unwrap()
    }
}

impl From<&Mail> for Vec<u8> {
    fn from(value: &Mail) -> Self {
        serde_json::to_vec(value).unwrap()
    }
}

impl From<&[u8]> for Mail {
    fn from(value: &[u8]) -> Self {
        serde_json::from_slice(value).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::state_machine::StateMachine;

    use super::*;

    #[test]
    fn test_regular_flow() {
        let mut sm = StateMachine::new("dummy");
        assert_eq!(sm.state, State::Fresh);
        sm.handle_smtp("HELO localhost").unwrap();
        assert_eq!(sm.state, State::Greeted);
        sm.handle_smtp("MAIL FROM: <local@example.com>").unwrap();
        assert!(matches!(sm.state, State::ReceivingRcpt(_)));
        sm.handle_smtp("RCPT TO: <a@localhost.com>").unwrap();
        assert!(matches!(sm.state, State::ReceivingRcpt(_)));
        sm.handle_smtp("RCPT TO: <b@localhost.com>").unwrap();
        assert!(matches!(sm.state, State::ReceivingRcpt(_)));
        sm.handle_smtp("DATA hello world\n").unwrap();
        assert!(matches!(sm.state, State::ReceivingData(_)));
        sm.handle_smtp("DATA hello world2\n").unwrap();
        assert!(matches!(sm.state, State::ReceivingData(_)));
        sm.handle_smtp("QUIT").unwrap();
        assert!(matches!(sm.state, State::Received(_)));
    }

    #[test]
    fn test_no_greeting() {
        let mut sm = StateMachine::new("dummy");
        assert_eq!(sm.state, State::Fresh);
        for command in [
            "MAIL FROM: <local@example.com>",
            "RCPT TO: <local@example.com>",
            "DATA hey",
            "GARBAGE",
        ] {
            assert!(sm.handle_smtp(command).is_err());
        }
    }
}
