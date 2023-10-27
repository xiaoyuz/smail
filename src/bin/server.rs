use std::process::exit;

use clap::Parser;
use slog::info;
use smail::{
    config::{
        config_tcp_listen_or_default, config_tcp_port_or_default, config_web_listen_or_default,
        config_web_port_or_default, set_global_config, Config,
    },
    server::Server,
    web::WebServer,
    Result, LOGGER,
};
use tokio::{fs, net::TcpListener, spawn};

const SMTP_DOMAIN: &str = "smtp.smail.my";

#[tokio::main]
pub async fn main() -> Result<()> {
    let cli = Cli::from_args();
    // Load config
    let mut config: Option<Config> = None;
    if let Some(config_file_name) = cli.config {
        let config_content = fs::read_to_string(config_file_name)
            .await
            .expect("Failed to read config file");

        // deserialize toml config
        config = match toml::from_str(&config_content) {
            Ok(d) => Some(d),
            Err(e) => {
                print!("Unable to load config file {e}");
                exit(1);
            }
        };
    };

    match &config {
        Some(c) => {
            println!("{c:?}");
            set_global_config(c.clone())
        }
        None => (),
    }

    start_server(
        &config_tcp_listen_or_default(),
        &config_tcp_port_or_default(),
    )
    .await?;

    start_web(
        &config_web_listen_or_default(),
        &config_web_port_or_default(),
    )
    .await?;

    Ok(())
}

async fn start_web(web_listen: &str, web_port: &str) -> Result<()> {
    let web_server = WebServer::new(format!("{}:{}", web_listen, web_port));
    info!(LOGGER, "web start, listen on port: {}", web_port);
    web_server.run().await?;
    Ok(())
}

async fn start_server(tcp_listen: &str, tcp_port: &str) -> Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", tcp_listen, tcp_port)).await?;

    info!(LOGGER, "mail server start, listen on port: {}", tcp_port);
    spawn(async move {
        loop {
            let (stream, addr) = listener.accept().await.unwrap();
            info!(LOGGER, "Accepted a connection from {}", addr);
            spawn(async move {
                let smtp = Server::new(SMTP_DOMAIN, stream).await?;
                smtp.serve().await
            });
        }
    });

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(name = "smail-server", version, author, about = "smail server")]
struct Cli {
    #[structopt(name = "config", long = "--config")]
    config: Option<String>,
}
