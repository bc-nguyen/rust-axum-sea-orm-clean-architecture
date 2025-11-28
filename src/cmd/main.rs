use lib::{
    config,
    infrastructure::db,
    presentation::{http::HttpServer, trace},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::AppConfig::from_env()?;

    trace::register();

    let pool = db::init_db(&config.db_connect_str).await?;

    let http_server = HttpServer::new(pool, config).await?;

    http_server.start().await
}
