use std::pin::Pin;

use axum::{
    Extension, Json,
    extract::State,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::{
    application::{PublicCase, SecureCase, error::AppError},
    presentation::{guards::UserInfo, http::AppState},
};

type HandlerFuture = Pin<Box<dyn Future<Output = Result<Response, AppError>> + Send>>;

pub fn secure_case_handler<U, I, O, F>(
    make_uc: F,
) -> impl Fn(State<AppState>, Extension<UserInfo>, I) -> HandlerFuture + Clone + Send + Sync + 'static
where
    U: SecureCase<Input = I, Output = O> + Send + Sync + 'static,
    I: Send + 'static,
    O: Serialize + Send + 'static,
    F: Fn(&AppState) -> U + Send + Sync + Clone + 'static,
{
    move |State(state), Extension(user), input| {
        let uc = make_uc(&state);

        Box::pin(async move {
            let result = uc.execute(input, user).await?;
            let response: Response = (result.status, Json(result.data)).into_response();
            Ok(response)
        })
    }
}

pub fn public_case_handler<U, I, O, F>(
    make_uc: F,
) -> impl Fn(State<AppState>, I) -> HandlerFuture + Clone + Send + Sync + 'static
where
    U: PublicCase<Input = I, Output = O> + Send + Sync + 'static,
    I: Send + 'static,
    O: Serialize + Send + 'static,
    F: Fn(&AppState) -> U + Send + Sync + Clone + 'static,
{
    move |State(state), input| {
        let uc = make_uc(&state);

        Box::pin(async move {
            let result = uc.execute(input).await?;
            let response: Response = (result.status, Json(result.data)).into_response();
            Ok(response)
        })
    }
}
