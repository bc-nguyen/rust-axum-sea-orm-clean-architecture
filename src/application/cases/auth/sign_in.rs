use async_trait::async_trait;

use crate::{
    application::{PublicCase, error::AppError},
    define_case,
    presentation::response::CaseResponse,
};

define_case!(SignInUseCase);

#[async_trait]
impl PublicCase for SignInUseCase {
    type Input = ();
    type Output = String;

    async fn execute(self, _: Self::Input) -> Result<CaseResponse<Self::Output>, AppError> {
        let token = self.state.jwt_helper.generate("aaa".to_string())?;

        Ok(CaseResponse::ok(token))
    }
}
