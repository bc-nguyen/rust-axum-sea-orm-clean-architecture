use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, ConnectionTrait};
use uuid::Uuid;

use crate::{
    application::dtos::department::ReqAddDepartmentDto,
    domain::{error::DomainError, organization::repositories::DepartmentRepository},
    infrastructure::db::{Repository, entities::departments},
};

#[async_trait]
impl<'a, C: ConnectionTrait> DepartmentRepository for Repository<'a, C> {
    async fn add(&self, dep: ReqAddDepartmentDto) -> Result<Uuid, DomainError> {
        let department = departments::ActiveModel::from(dep);

        let department = department.insert(self.db).await?;

        Ok(department.id)
    }
}
