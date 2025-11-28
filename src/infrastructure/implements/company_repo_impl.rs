use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    application::dtos::company::{ReqAddCompanyDto, ReqQueryCompanyDto, ResQueryCompanyDto},
    domain::{error::DomainError, organization::repositories::CompanyRepository},
    infrastructure::db::{Repository, entities::companies},
};

#[async_trait]
impl<'a, C: ConnectionTrait> CompanyRepository for Repository<'a, C> {
    async fn query(
        &self,
        cond: &ReqQueryCompanyDto,
    ) -> Result<Vec<ResQueryCompanyDto>, DomainError> {
        let mut query = companies::Entity::find();
        if let Some(name) = &cond.name
            && !name.is_empty()
        {
            query = query.filter(companies::Column::Name.like(format!("%{}%", name)));
        }

        let result = query.all(self.db).await?;

        let result = result.into_iter().map(|c| c.into()).collect();

        Ok(result)
    }

    async fn exists(&self, id: Uuid) -> Result<bool, DomainError> {
        let result = companies::Entity::find_by_id(id).one(self.db).await?;

        Ok(result.is_some())
    }

    async fn add(&self, com: ReqAddCompanyDto) -> Result<Uuid, DomainError> {
        let company = companies::ActiveModel::from(com);

        let company = company.insert(self.db).await?;

        Ok(company.id)
    }
}
