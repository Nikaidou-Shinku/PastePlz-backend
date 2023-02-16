pub use sea_orm_migration::prelude::*;

mod m20230216_000001_create_paste_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![Box::new(m20230216_000001_create_paste_table::Migration)]
  }
}
