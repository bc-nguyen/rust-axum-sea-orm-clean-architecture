use async_trait::async_trait;

use crate::{
    application::{SecureCase, dtos::company::ReqAddCompanyDto, error::AppError},
    define_case,
    domain::organization::repositories::CompanyRepository,
    presentation::{guards::UserInfo, middlewares::validator::JsonParams, response::CaseResponse},
};

define_case!(AddCompanyUseCase);

#[async_trait]
impl SecureCase for AddCompanyUseCase {
    type Input = JsonParams<ReqAddCompanyDto>;
    type Output = String;

    async fn execute(
        self,
        JsonParams(dto): JsonParams<ReqAddCompanyDto>,
        _user: UserInfo,
    ) -> Result<CaseResponse<String>, AppError> {
        tracing::debug!("dto: {:?}", dto);

        let provider = self.state.db_context.provider();
        let repo = provider.company_repo();
        let id = repo.add(dto).await?;

        Ok(CaseResponse::created(id.to_string()))
    }
}
