pub use sea_orm_migration::prelude::*;

mod m20251119_070234_create_company_table;
mod m20251119_070643_create_department_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251119_070234_create_company_table::Migration),
            Box::new(m20251119_070643_create_department_table::Migration),
        ]
    }
}
