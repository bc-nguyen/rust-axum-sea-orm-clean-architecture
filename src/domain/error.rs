use axum::http::StatusCode;
use sea_orm::DbErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error(transparent)]
    DbError(#[from] DbErr),
    #[error("use case error: [{1}]{2}")]
    CaseError(StatusCode, String, String),
}
