use std::sync::Arc;

use async_once::AsyncOnce;
use lazy_static::lazy_static;
use mapuche_embedded::{
    cmd::{Command, Mget, Set, Zadd, Zrange},
    frame::Frame,
    Conn, OpenOptions, DB,
};

use uuid::Uuid;

use crate::{config::data_store_dir_or_default, utils::current_time_mills, Mail, Result};

lazy_static! {
    pub static ref LOCAL_STORAGE: AsyncOnce<Arc<Storage>> =
        AsyncOnce::new(async { Arc::new(new_storage().await) });
}

async fn new_storage() -> Storage {
    Storage::open().await
}

pub struct Storage {
    db: DB,
}

impl Storage {
    pub async fn open() -> Self {
        let options = OpenOptions::new();
        let db = options.open(data_store_dir_or_default()).await.unwrap();
        Storage { db }
    }

    pub fn conn(&self) -> Conn {
        self.db.conn()
    }
}

pub struct MailRepository {
    storage: Arc<Storage>,
}

impl MailRepository {
    pub fn new(storage: Arc<Storage>) -> Self {
        Self { storage }
    }

    pub async fn replicate(&self, mut mail: Mail) -> Result<()> {
        let now = current_time_mills();
        let conn = self.storage.conn();

        if mail.id.is_empty() {
            mail.id = Uuid::new_v4().to_string();
        }
        mail.ts = now;

        let mail_data: String = (&mail).into();

        // Set the mail data with id as key
        let cmd = Command::Set(Set::new(&mail.id, mail_data, None, None));
        conn.execute(cmd).await?;

        // Set 'mail to' queue
        for to in &mail.to {
            let cmd = Command::Zadd(Zadd::new(
                to_key(to),
                &[&mail.id],
                &[now as f64],
                None,
                false,
            ));
            conn.execute(cmd).await?;
        }

        Ok(())
    }

    pub async fn query_mails(&self, to: &str, offset: i64, size: i64) -> Option<Vec<Mail>> {
        let conn = self.storage.conn();
        let cmd = Command::Zrange(Zrange::new(to_key(to), offset, offset + size, false, true));
        let ids: Vec<String> = if let Frame::Array(frames) = conn.execute(cmd).await.ok()? {
            frames
                .into_iter()
                .map(|frame| {
                    if let Frame::Bulk(bs) = frame {
                        let bs: Vec<u8> = bs.into();
                        return Some(String::from_utf8(bs).unwrap_or_default());
                    }
                    None
                })
                .filter(|item| item.is_some())
                .map(|item| item.unwrap_or_default())
                .collect()
        } else {
            Default::default()
        };

        if ids.is_empty() {
            return None;
        }

        let cmd = Command::Mget(Mget::new(ids.as_slice()));
        if let Frame::Array(frames) = conn.execute(cmd).await.ok()? {
            let res = frames
                .into_iter()
                .map(|frame| {
                    if let Frame::Bulk(bs) = frame {
                        let bs: Vec<u8> = bs.into();
                        return Some(bs.as_slice().into());
                    }
                    None
                })
                .filter(|item| item.is_some())
                .map(|item| item.unwrap_or_default())
                .collect();
            Some(res)
        } else {
            None
        }
    }
}

fn to_key(to: &str) -> String {
    format!("TO_{}", to)
}
