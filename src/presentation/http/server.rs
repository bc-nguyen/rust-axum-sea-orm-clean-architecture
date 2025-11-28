use std::sync::Arc;

use crate::{
    config::AppConfig,
    infrastructure::{db::DbContext, helpers::token::JwtHelper},
    presentation::http::AppState,
};
use anyhow::Context;
use axum::{Router, extract::Request, routing::get, serve};
use sea_orm::DatabaseConnection;

use super::routes;

pub struct HttpServer {
    router: Router,
    listener: tokio::net::TcpListener,
}

impl HttpServer {
    pub async fn new(db: Arc<DatabaseConnection>, config: Arc<AppConfig>) -> anyhow::Result<Self> {
        let trace_layer =
            tower_http::trace::TraceLayer::new_for_http().make_span_with(|req: &Request<_>| {
                let uri = req.uri().to_string();
                tracing::info_span!("[HTTP]", method = ?req.method(), uri)
            });

        let state = AppState {
            db_context: Arc::new(DbContext::new(db)),
            jwt_helper: Arc::new(JwtHelper::new(config.token_secret_key.clone())),
        };

        let router = Router::new()
            .nest(
                "/api/",
                api_routes().nest("/v1", routes::v1::v1_routes(state.clone())),
            )
            .layer(trace_layer)
            .with_state(state);

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.server_port))
            .await
            .with_context(|| format!("failed to start listening on {}", config.server_port))?;

        Ok(Self { router, listener })
    }

    pub async fn start(self) -> anyhow::Result<()> {
        tracing::debug!("listening on {}", self.listener.local_addr().unwrap());
        serve(self.listener, self.router)
            .await
            .context("received error from running server")?;
        Ok(())
    }
}

fn api_routes() -> Router<AppState> {
    Router::new().route("/healthy", get(|| async { "I'm alive!" }))
}
