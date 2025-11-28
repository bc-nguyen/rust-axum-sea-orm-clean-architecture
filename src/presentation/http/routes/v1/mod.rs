use crate::{
    application::cases::{
        auth::SignInUseCase,
        company::{AddCompanyUseCase, QueryCompanyUseCase},
        department::AddDepartmentUseCase,
    },
    make_case,
    presentation::{
        guards,
        handlers::{public_case_handler, secure_case_handler},
        http::AppState,
    },
};

use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, post},
};

pub fn v1_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/companies",
            get(secure_case_handler(make_case!(QueryCompanyUseCase)))
                .route_layer(from_fn_with_state(vec!["admin".to_string()], guards::roles))
                .post(secure_case_handler(make_case!(AddCompanyUseCase))),
        )
        .route(
            "/departments",
            post(secure_case_handler(make_case!(AddDepartmentUseCase))),
        )
        .layer(from_fn_with_state(state, guards::auth))
        .route(
            "/signin",
            get(public_case_handler(make_case!(SignInUseCase))),
        )
}
