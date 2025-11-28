use async_trait::async_trait;
use serde::Serialize;

use crate::{
    application::error::AppError,
    presentation::{guards::UserInfo, response::CaseResponse},
};

#[async_trait]
pub trait PublicCase {
    type Input;
    type Output: Serialize;

    async fn execute(self, dto: Self::Input) -> Result<CaseResponse<Self::Output>, AppError>;
}

#[async_trait]
pub trait SecureCase {
    type Input;
    type Output: Serialize;

    async fn execute(
        self,
        dto: Self::Input,
        user: UserInfo,
    ) -> Result<CaseResponse<Self::Output>, AppError>;
}

#[macro_export]
macro_rules! make_case {
    ($use_case:ty) => {
        |state: &AppState| {
            use std::sync::Arc;

            <$use_case>::new(Arc::new(state.clone()))
        }
    };
}

#[macro_export]
macro_rules! define_case {
    ($uc:ident) => {
        use std::sync::Arc;
        use $crate::presentation::http::AppState;

        #[derive(Debug)]
        pub struct $uc {
            pub state: Arc<AppState>,
        }

        impl $uc {
            pub fn new(state: std::sync::Arc<AppState>) -> Self {
                Self { state }
            }
        }
    };
}
