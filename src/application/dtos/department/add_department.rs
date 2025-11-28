use sea_orm::ActiveValue::Set;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::infrastructure::db::entities::departments;

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct ReqAddDepartmentDto {
    #[validate(length(min = 1, message = "add.department.name.required"))]
    pub name: String,
    pub company_id: Uuid,
}

impl From<ReqAddDepartmentDto> for departments::ActiveModel {
    fn from(value: ReqAddDepartmentDto) -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            name: Set(value.name),
            company_id: Set(value.company_id),
        }
    }
}
