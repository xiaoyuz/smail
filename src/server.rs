use slog::{debug, info};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{
    db::{MailRepository, LOCAL_STORAGE},
    Result, LOGGER,
};

use super::{state_machine::StateMachine, State};

pub struct Server {
    stream: TcpStream,
    state_machine: StateMachine,
    repository: MailRepository,
}

impl Server {
    /// Creates a new server from a connected stream
    pub async fn new(domain: impl AsRef<str>, stream: TcpStream) -> Result<Self> {
        Ok(Self {
            stream,
            state_machine: StateMachine::new(domain),
            repository: MailRepository::new(LOCAL_STORAGE.get().await.clone()),
        })
    }

    pub async fn serve(mut self) -> Result<()> {
        self.greet().await?;

        let mut buf = vec![0; 65536];
        loop {
            let n = self.stream.read(&mut buf).await?;

            if n == 0 {
                info!(LOGGER, "Received EOF");
                self.state_machine.handle_smtp("quit").ok();
                break;
            }
            let msg = std::str::from_utf8(&buf[0..n])?;
            let response = self.state_machine.handle_smtp(msg)?;
            if response != StateMachine::HOLD_YOUR_HORSES {
                self.stream.write_all(response).await?;
            } else {
                debug!(LOGGER, "Not responding, awaiting more data");
            }
            if response == StateMachine::KTHXBYE {
                break;
            }
        }
        match self.state_machine.state {
            State::Received(mail) => {
                self.repository.replicate(mail).await?;
            }
            State::ReceivingData(mail) => {
                info!(LOGGER, "Received EOF before receiving QUIT");
                self.repository.replicate(mail).await?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Sends the initial SMTP greeting
    async fn greet(&mut self) -> Result<()> {
        self.stream
            .write_all(StateMachine::OH_HAI)
            .await
            .map_err(|e| e.into())
    }
}
