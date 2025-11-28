use async_trait::async_trait;

use crate::{
    application::{
        SecureCase,
        dtos::company::{ReqQueryCompanyDto, ResQueryCompanyDto},
        error::AppError,
    },
    define_case,
    domain::organization::repositories::CompanyRepository,
    presentation::{guards::UserInfo, middlewares::validator::QueryParams, response::CaseResponse},
};

define_case!(QueryCompanyUseCase);

#[async_trait]
impl SecureCase for QueryCompanyUseCase {
    type Input = QueryParams<ReqQueryCompanyDto>;
    type Output = Vec<ResQueryCompanyDto>;

    async fn execute(
        self,
        QueryParams(dto): QueryParams<ReqQueryCompanyDto>,
        _user: UserInfo,
    ) -> Result<CaseResponse<Vec<ResQueryCompanyDto>>, AppError> {
        tracing::debug!("dto: {:?} user: {:?}", dto, _user);

        let provider = self.state.db_context.provider();
        let repo = provider.company_repo();

        let companies = repo.query(&dto).await?;

        Ok(CaseResponse::ok(companies))
    }
}
