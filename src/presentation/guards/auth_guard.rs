use axum::{
    extract::{Request, State},
    http,
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};

use crate::{application::error::AppError, presentation::http::AppState};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub id: String,
}

#[derive(Debug)]
pub enum AuthStatus {
    Authenticated(UserInfo),
    Anonymous,
}

pub async fn auth(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let status = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .and_then(|token| state.jwt_helper.validate(token).ok())
        .map(|v| AuthStatus::Authenticated(super::UserInfo { id: v }))
        .unwrap_or(AuthStatus::Anonymous);

    match status {
        AuthStatus::Anonymous => Err(AppError::UnAuthorized("token invalid or expired.".to_string())),
        AuthStatus::Authenticated(user) => {
            req.extensions_mut().insert(user);
            Ok(next.run(req).await)
        }
    }
}
