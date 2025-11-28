use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    application::dtos::company::{ReqAddCompanyDto, ReqQueryCompanyDto, ResQueryCompanyDto},
    domain::error::DomainError,
};

#[async_trait]
pub trait CompanyRepository {
    async fn query(
        &self,
        cond: &ReqQueryCompanyDto,
    ) -> Result<Vec<ResQueryCompanyDto>, DomainError>;
    async fn exists(&self, id: Uuid) -> Result<bool, DomainError>;
    async fn add(&self, com: ReqAddCompanyDto) -> Result<Uuid, DomainError>;
}
