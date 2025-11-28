use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("departments")
                    .if_not_exists()
                    .col(pk_uuid("id"))
                    .col(string_len_uniq("name", 200).not_null())
                    .col(uuid("company_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Alias::new("departments"), Alias::new("company_id"))
                            .to(Alias::new("companies"), Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("departments").to_owned())
            .await
    }
}
