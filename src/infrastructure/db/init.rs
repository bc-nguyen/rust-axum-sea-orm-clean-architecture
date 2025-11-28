use std::sync::Arc;

use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

pub async fn init_db(db_connect_str: &str) -> Result<Arc<DatabaseConnection>, anyhow::Error> {
    let db = Database::connect(db_connect_str).await?;

    Migrator::up(&db, None).await?;

    Ok(Arc::new(db))
}
