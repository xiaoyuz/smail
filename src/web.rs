use actix_files as fs;
use actix_web::{
    get, post,
    web::{Data, Json},
    App, HttpResponse, HttpServer, Responder,
};
use serde::Deserialize;
use slog::info;

use crate::{
    db::{MailRepository, LOCAL_STORAGE},
    utils::current_time_mills,
    Mail, LOGGER,
};

pub struct WebServer {
    listen_addr: String,
}

impl WebServer {
    pub fn new(listen_addr: String) -> Self {
        WebServer { listen_addr }
    }

    pub async fn run(self) -> std::io::Result<()> {
        let addr = self.listen_addr.clone();
        info!(LOGGER, "Web Server Listen on: {}", addr);

        let arc_state = Data::new(self);
        // Start the actix-web server.
        let server = HttpServer::new(move || {
            App::new()
                .app_data(arc_state.clone())
                .service(echo)
                .service(query_mails)
                .service(test_data)
                .service(fs::Files::new("/", "static").index_file("index.html"))
        });

        let x = server.bind(addr)?;
        x.run().await
    }
}

#[derive(Deserialize)]
struct MailQuery {
    to: String,
    offset: i64,
    size: i64,
}

#[get("/echo")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body("echo")
}

#[post("/query_mails")]
async fn query_mails(info: Json<MailQuery>) -> impl Responder {
    let repository = MailRepository::new(LOCAL_STORAGE.get().await.clone());
    if let Some(mails) = repository
        .query_mails(&info.to, info.offset, info.size)
        .await
    {
        HttpResponse::Ok().json(mails)
    } else {
        let empty: Vec<Mail> = Default::default();
        HttpResponse::Ok().json(empty)
    }
}

#[get("/test_data")]
async fn test_data() -> impl Responder {
    let repository = MailRepository::new(LOCAL_STORAGE.get().await.clone());
    repository
        .replicate(Mail {
            id: String::from(""),
            from: String::from("abc@smail.my"),
            to: vec![String::from("test@smail.my")],
            data: String::from("Hello1"),
            ts: current_time_mills(),
        })
        .await
        .unwrap();
    repository
        .replicate(Mail {
            id: String::from(""),
            from: String::from("123@smail.my"),
            to: vec![String::from("test@smail.my")],
            data: String::from("Hello2"),
            ts: current_time_mills(),
        })
        .await
        .unwrap();
    repository
        .replicate(Mail {
            id: String::from(""),
            from: String::from("zzz@smail.my"),
            to: vec![String::from("test@smail.my")],
            data: String::from("Hello3"),
            ts: current_time_mills(),
        })
        .await
        .unwrap();

    HttpResponse::Ok().body("test mails added")
}
