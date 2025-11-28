use axum::{
    Json,
    extract::rejection::{FormRejection, JsonRejection, PathRejection, QueryRejection},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::DbErr;
use serde::Serialize;
use thiserror::Error;

use crate::domain::error::DomainError;

#[derive(Debug, Error, Serialize)]
#[error("[{code}]{message}")]
struct ErrorData {
    code: String,
    message: String,
}

impl ErrorData {
    fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    FormRejection(#[from] FormRejection),
    #[error(transparent)]
    PathRejection(#[from] PathRejection),
    #[error(transparent)]
    QueryRejection(#[from] QueryRejection),
    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
    #[error("{0}")]
    UnAuthorized(String),
    #[error("{0}")]
    Forbidden(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        use AppError::*;

        match self {
            Forbidden(c) => (
                StatusCode::UNAUTHORIZED,
                Json(ResponseBody::new(
                    StatusCode::FORBIDDEN,
                    ErrorData::new("FORBIDDEN", c.as_str()),
                )),
            )
                .into_response(),
            UnAuthorized(c) => (
                StatusCode::UNAUTHORIZED,
                Json(ResponseBody::new(
                    StatusCode::UNAUTHORIZED,
                    ErrorData::new("UNAUTHORIZED", c.as_str()),
                )),
            )
                .into_response(),
            ValidationError(_) => {
                tracing::error!("{}", self);
                (
                    StatusCode::BAD_REQUEST,
                    Json(ResponseBody::new(
                        StatusCode::BAD_REQUEST,
                        ErrorData::new("INPUT_VALIDATE_FAIL", self.to_string().as_str()),
                    )),
                )
                    .into_response()
            }

            FormRejection(_) | PathRejection(_) | QueryRejection(_) | JsonRejection(_) => {
                tracing::error!("{}", self);
                (
                    StatusCode::BAD_REQUEST,
                    Json(ResponseBody::new(
                        StatusCode::BAD_REQUEST,
                        ErrorData::new("INPUT_PARSE_FAIL", self.to_string().as_str()),
                    )),
                )
                    .into_response()
            }

            Domain(d) => {
                tracing::error!("{}", d);
                match d {
                    DomainError::DbError(e) => {
                        match e {
                            DbErr::RecordNotFound(_) => (
                                StatusCode::NOT_FOUND,
                                Json(ResponseBody::new(
                                    StatusCode::NOT_FOUND,
                                    ErrorData::new("DATA_DUPPLICATED", e.to_string().as_str()),
                                )),
                            ),
                            DbErr::RecordNotInserted => (
                                StatusCode::CONFLICT,
                                Json(ResponseBody::new(
                                    StatusCode::CONFLICT,
                                    ErrorData::new("DATA_DUPPLICATED", e.to_string().as_str()),
                                )),
                            ),
                            _ => (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(ResponseBody::new(
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    ErrorData::new("DB_ERROR", e.to_string().as_str()),
                                )),
                            ),
                        }
                    }
                    .into_response(),

                    DomainError::CaseError(s, c, m) => (
                        s,
                        Json(ResponseBody::new(s, ErrorData::new(&c, m.as_str()))),
                    )
                        .into_response(),
                }
            }

            InternalError(_) => {
                tracing::error!("{}", self);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ResponseBody::new(
                        StatusCode::BAD_REQUEST,
                        ErrorData::new("UNKNOWN_INTERNAL_ERROR", self.to_string().as_str()),
                    )),
                )
                    .into_response()
            }
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct ResponseBody<T: Serialize> {
    status_code: u16,
    data: T,
}

impl<T: Serialize> ResponseBody<T> {
    fn new(status_code: StatusCode, data: T) -> Self {
        Self {
            status_code: status_code.as_u16(),
            data,
        }
    }
}
