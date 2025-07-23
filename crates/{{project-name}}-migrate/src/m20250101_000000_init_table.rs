use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

use super::iden::account::Account;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table = Table::create()
            .table(Account::Table)
            .if_not_exists()
            .col(pk_uuid(Account::ID))
            .to_owned();
        manager.create_table(timestamps(table)).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Account::Table).to_owned()).await?;
        Ok(())
    }
}
