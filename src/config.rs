use anyhow::Context;
use std::{env, sync::Arc};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_port: String,
    pub db_connect_str: String,
    pub token_secret_key: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Arc<Self>> {
        let server_port = load_env("SERVER_PORT").unwrap_or("8080".to_string());
        let db_connect_str = load_env("DB_CONNECT_STR")?;
        let token_secret_key = load_env("TOKEN_SECRET_KEY")?;

        Ok(Arc::new(Self {
            server_port,
            db_connect_str,
            token_secret_key,
        }))
    }
}

fn load_env(key: &str) -> anyhow::Result<String> {
    env::var(key).with_context(|| format!("failed to load environment variable {}", key))
}
