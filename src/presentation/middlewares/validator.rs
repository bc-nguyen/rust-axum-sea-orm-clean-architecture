use axum::{
    Form, Json,
    extract::{FromRequest, FromRequestParts, Path, Query, Request, rejection::FormRejection},
    http::request::Parts,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::application::error::AppError;

#[derive(Debug, Clone, Copy, Default)]
pub struct FormParams<T>(pub T);

impl<T, S> FromRequest<S> for FormParams<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PathParams<T>(pub T);

impl<T, S> FromRequestParts<S> for PathParams<T>
where
    T: DeserializeOwned + Validate + Send,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let axum::extract::Path(value) =
            axum::extract::Path::<T>::from_request_parts(parts, state).await?;
        value.validate()?;
        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct JsonParams<T>(pub T);

impl<T, S> FromRequest<S> for JsonParams<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct QueryParams<T>(pub T);

impl<T, S> FromRequest<S> for QueryParams<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AllParams<P, Q, B> {
    pub p: P,
    pub q: Q,
    pub b: B,
}

impl<P, Q, B, S> FromRequest<S> for AllParams<P, Q, B>
where
    P: DeserializeOwned + Validate + Send,
    Q: DeserializeOwned + Validate + Send,
    B: DeserializeOwned + Validate + Send,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();

        let Path(p) = Path::<P>::from_request_parts(&mut parts, state).await?;
        p.validate()?;

        let Query(q) = Query::<Q>::from_request_parts(&mut parts, state).await?;
        q.validate()?;

        let origin_req = Request::from_parts(parts, body);

        let Json(b) = Json::<B>::from_request(origin_req, state).await?;
        b.validate()?;

        Ok(Self { p, q, b })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PathAndJsonParams<P, B> {
    pub p: P,
    pub b: B,
}

impl<P, B, S> FromRequest<S> for PathAndJsonParams<P, B>
where
    P: DeserializeOwned + Validate + Send,
    B: DeserializeOwned + Validate + Send,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();

        let Path(p) = Path::<P>::from_request_parts(&mut parts, state).await?;
        p.validate()?;

        let origin_req = Request::from_parts(parts, body);

        let Json(b) = Json::<B>::from_request(origin_req, state).await?;
        b.validate()?;

        Ok(Self { p, b })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct QueryAndJsonParams<Q, B> {
    pub q: Q,
    pub b: B,
}

impl<Q, B, S> FromRequest<S> for QueryAndJsonParams<Q, B>
where
    Q: DeserializeOwned + Validate + Send,
    B: DeserializeOwned + Validate + Send,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();

        let Query(q) = Query::<Q>::from_request_parts(&mut parts, state).await?;
        q.validate()?;

        let origin_req = Request::from_parts(parts, body);

        let Json(b) = Json::<B>::from_request(origin_req, state).await?;
        b.validate()?;

        Ok(Self { q, b })
    }
}
