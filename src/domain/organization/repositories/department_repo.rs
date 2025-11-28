use async_trait::async_trait;
use uuid::Uuid;

use crate::{application::dtos::department::ReqAddDepartmentDto, domain::error::DomainError};

#[async_trait]
pub trait DepartmentRepository {
    async fn add(&self, dep: ReqAddDepartmentDto) -> Result<Uuid, DomainError>;
}
