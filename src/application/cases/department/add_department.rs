use async_trait::async_trait;

use crate::{
    application::{SecureCase, dtos::department::ReqAddDepartmentDto, error::AppError},
    define_case,
    domain::{
        error::DomainError,
        organization::repositories::{CompanyRepository, DepartmentRepository},
    },
    presentation::{guards::UserInfo, middlewares::validator::JsonParams, response::CaseResponse},
    with_transaction,
};

define_case!(AddDepartmentUseCase);

#[async_trait]
impl SecureCase for AddDepartmentUseCase {
    type Input = JsonParams<ReqAddDepartmentDto>;
    type Output = String;

    async fn execute(
        self,
        JsonParams(dto): JsonParams<ReqAddDepartmentDto>,
        _user: UserInfo,
    ) -> Result<CaseResponse<String>, AppError> {
        tracing::debug!("dto: {:?}", dto);

        let new_dep = with_transaction!(self.state.db_context, provider => {

            let com_exists = provider.company_repo().exists(dto.company_id).await?;

            if !com_exists {
                return Err(DomainError::DbError(sea_orm::DbErr::RecordNotFound(format!("company with id: {} is not found", dto.company_id))));
            }

            let dep_repo = provider.department_repo();

            let id = dep_repo.add(dto).await?;


            Ok(id.to_string())
        })?;

        Ok(CaseResponse::created(new_dep))
    }
}
