use sea_orm::ActiveValue::Set;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::infrastructure::db::entities::companies;

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct ReqAddCompanyDto {
    #[validate(length(
        min = 1,
        max = 200,
        message = "company's is required and max 200 characters."
    ))]
    pub name: String,
}

impl From<ReqAddCompanyDto> for companies::ActiveModel {
    fn from(value: ReqAddCompanyDto) -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            name: Set(value.name),
        }
    }
}
