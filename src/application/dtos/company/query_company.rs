use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::infrastructure::db::entities::companies;

#[derive(Debug, Deserialize, Validate)]
pub struct ReqQueryCompanyDto {
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct ResQueryCompanyDto {
    pub id: String,
    pub name: String,
}

impl From<companies::Model> for ResQueryCompanyDto {
    fn from(c: companies::Model) -> Self {
        Self {
            id: c.id.to_string(),
            name: c.name,
        }
    }
}
