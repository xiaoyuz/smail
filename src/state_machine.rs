use crate::{Mail, Result, SmailError, State, LOGGER};
use anyhow::Context;
use slog::{debug, info, warn};

#[derive(Clone, Debug)]
pub struct StateMachine {
    pub state: State,
    pub ehlo_greeting: String,
}

/// An state machine capable of handling SMTP commands
/// for receiving mail.
/// Use handle_smtp() to handle a single command.
/// The return value from handle_smtp() is the response
/// that should be sent back to the client.
impl StateMachine {
    pub const OH_HAI: &[u8] = b"220 edgemail\n";
    pub const KK: &[u8] = b"250 Ok\n";
    pub const AUTH_OK: &[u8] = b"235 Ok\n";
    pub const SEND_DATA_PLZ: &[u8] = b"354 End data with <CR><LF>.<CR><LF>\n";
    pub const KTHXBYE: &[u8] = b"221 Bye\n";
    pub const HOLD_YOUR_HORSES: &[u8] = &[];

    pub fn new(domain: impl AsRef<str>) -> Self {
        let domain = domain.as_ref();
        let ehlo_greeting = format!("250-{domain} Hello {domain}\n250 AUTH PLAIN LOGIN\n");
        Self {
            state: State::Fresh,
            ehlo_greeting,
        }
    }

    /// Handles a single SMTP command and returns a proper SMTP response
    pub fn handle_smtp(&mut self, raw_msg: &str) -> Result<&[u8]> {
        info!(LOGGER, "Received {raw_msg} in state {:?}", self.state);
        let mut msg = raw_msg.split_whitespace();
        let command = msg.next().context("received empty command")?.to_lowercase();
        let state = std::mem::replace(&mut self.state, State::Fresh);
        match (command.as_str(), state) {
            ("ehlo", State::Fresh) => {
                info!(LOGGER, "Sending AUTH info");
                self.state = State::Greeted;
                Ok(self.ehlo_greeting.as_bytes())
            }
            ("helo", State::Fresh) => {
                self.state = State::Greeted;
                Ok(StateMachine::KK)
            }
            ("noop", _) | ("help", _) | ("info", _) | ("vrfy", _) | ("expn", _) => {
                info!(LOGGER, "Got {command}");
                Ok(StateMachine::KK)
            }
            ("rset", _) => {
                self.state = State::Fresh;
                Ok(StateMachine::KK)
            }
            ("auth", _) => {
                info!(LOGGER, "Acknowledging AUTH");
                Ok(StateMachine::AUTH_OK)
            }
            ("mail", State::Greeted) => {
                info!(LOGGER, "Receiving MAIL");
                let from = msg.next().context("received empty MAIL")?;
                let from = from
                    .strip_prefix("FROM:")
                    .context("received incorrect MAIL")?;
                info!(LOGGER, "FROM: {from}");
                self.state = State::ReceivingRcpt(Mail {
                    from: from.to_string(),
                    ..Default::default()
                });
                Ok(StateMachine::KK)
            }
            ("rcpt", State::ReceivingRcpt(mut mail)) => {
                info!(LOGGER, "Receiving rcpt");
                let to = msg.next().context("received empty RCPT")?;
                let to = to.strip_prefix("TO:").context("received incorrect RCPT")?;
                debug!(LOGGER, "TO: {to}");
                mail.to.push(to.to_string());
                self.state = State::ReceivingRcpt(mail);
                Ok(StateMachine::KK)
            }
            ("data", State::ReceivingRcpt(mail)) => {
                info!(LOGGER, "Receiving data");
                self.state = State::ReceivingData(mail);
                Ok(StateMachine::SEND_DATA_PLZ)
            }
            ("quit", State::ReceivingData(mail)) => {
                info!(
                    LOGGER,
                    "Received data: FROM: {} TO:{} DATA:{}",
                    mail.from,
                    mail.to.join(", "),
                    mail.data
                );
                self.state = State::Received(mail);
                Ok(StateMachine::KTHXBYE)
            }
            ("quit", _) => {
                warn!(LOGGER, "Received quit before getting any data");
                Ok(StateMachine::KTHXBYE)
            }
            (_, State::ReceivingData(mut mail)) => {
                info!(LOGGER, "Receiving data");
                let resp = if raw_msg.ends_with("\r\n.\r\n") {
                    StateMachine::KK
                } else {
                    StateMachine::HOLD_YOUR_HORSES
                };
                mail.data += raw_msg;
                self.state = State::ReceivingData(mail);
                Ok(resp)
            }
            _ => Err(SmailError::Owned(format!(
                "Unexpected message received in state {:?}",
                self.state
            ))
            .into()),
        }
    }
}
