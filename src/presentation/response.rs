use axum::http::StatusCode;
use serde::Serialize;

pub struct CaseResponse<T: Serialize> {
    pub status: StatusCode,
    pub data: T,
}

impl<T: Serialize> CaseResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            status: StatusCode::OK,
            data,
        }
    }

    pub fn created(data: T) -> Self {
        Self {
            status: StatusCode::CREATED,
            data,
        }
    }

    pub fn no_content() -> CaseResponse<()> {
        CaseResponse {
            status: StatusCode::NO_CONTENT,
            data: (),
        }
    }
}
