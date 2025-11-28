use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use crate::{application::error::AppError, presentation::guards::UserInfo};

pub async fn roles(
    State(required_roles): State<Vec<String>>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let user = req.extensions().get::<UserInfo>();
    tracing::debug!("user: {:?}, required_roles: {:?}", user, required_roles);
    //TODO: implement roles check here
    match user {
        None => Err(AppError::Forbidden("miss.permission".to_string())),
        Some(_) => Ok(next.run(req).await),
    }
}
