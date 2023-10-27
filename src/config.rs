use serde::Deserialize;

pub const DEFAULT_WEB_PORT: &str = "8631";
pub const DEFAULT_TCP_PORT: &str = "8222";

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    web_listen: Option<String>,
    web_port: Option<u16>,
    tcp_listen: Option<String>,
    tcp_port: Option<u16>,
    data_store_dir: Option<String>,
    log_level: Option<String>,
    log_file: Option<String>,
}

// Config
pub static mut SERVER_CONFIG: Option<Config> = None;

pub fn set_global_config(config: Config) {
    unsafe {
        SERVER_CONFIG.replace(config);
    }
}

pub fn data_store_dir_or_default() -> String {
    unsafe {
        if let Some(c) = &SERVER_CONFIG {
            if let Some(b) = c.data_store_dir.clone() {
                return b;
            }
        }
    }
    "./smail_server_store".to_owned()
}

fn log_level_str() -> String {
    unsafe {
        if let Some(c) = &SERVER_CONFIG {
            if let Some(l) = c.log_level.clone() {
                return l;
            }
        }
    }
    "info".to_owned()
}

pub fn log_level() -> usize {
    let level_str = log_level_str();
    match level_str.as_str() {
        "off" => 0,
        "critical" => 1,
        "error" => 2,
        "warning" => 3,
        "info" => 4,
        "debug" => 5,
        "trace" => 6,
        _ => 0,
    }
}

pub fn log_file() -> String {
    unsafe {
        if let Some(c) = &SERVER_CONFIG {
            if let Some(l) = c.log_file.clone() {
                return l;
            }
        }
    }
    "smail-server.log".to_owned()
}

pub fn config_web_listen_or_default() -> String {
    unsafe {
        if let Some(c) = &SERVER_CONFIG {
            if let Some(s) = c.web_listen.clone() {
                return s;
            }
        }
    }

    "0.0.0.0".to_owned()
}

pub fn config_web_port_or_default() -> String {
    unsafe {
        if let Some(c) = &SERVER_CONFIG {
            if let Some(s) = c.web_port {
                return s.to_string();
            }
        }
    }

    DEFAULT_WEB_PORT.to_owned()
}

pub fn config_tcp_listen_or_default() -> String {
    unsafe {
        if let Some(c) = &SERVER_CONFIG {
            if let Some(s) = c.tcp_listen.clone() {
                return s;
            }
        }
    }

    "0.0.0.0".to_owned()
}

pub fn config_tcp_port_or_default() -> String {
    unsafe {
        if let Some(c) = &SERVER_CONFIG {
            if let Some(s) = c.tcp_port {
                return s.to_string();
            }
        }
    }

    DEFAULT_TCP_PORT.to_owned()
}
