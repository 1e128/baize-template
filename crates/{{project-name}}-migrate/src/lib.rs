use baizekit::db::migration::*;
use sea_orm_migration::prelude::*;

mod iden;
mod m20250101_000000_init_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250101_000000_init_table::Migration)]
    }

    fn migration_table_name() -> DynIden {
        Alias::new("{{db_prefix}}_migrations").into_iden()
    }
}

baizekit::db::define_sea_orm_cli!(Migrator, Migrator);
