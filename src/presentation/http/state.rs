use std::sync::Arc;

use crate::infrastructure::{db::DbContext, helpers::token::JwtHelper};

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_context: Arc<DbContext>,
    pub jwt_helper: Arc<JwtHelper>,
}
