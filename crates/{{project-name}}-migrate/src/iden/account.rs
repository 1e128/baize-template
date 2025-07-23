use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Account {
    #[sea_orm(iden = "{{db_prefix}}_account")]
    Table,
    ID,
}
