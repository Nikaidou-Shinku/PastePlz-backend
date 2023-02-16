use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Paste::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(Paste::Token)
              .string_len(8)
              .not_null()
              .primary_key(),
          )
          .col(ColumnDef::new(Paste::Lang).string_len(8).not_null())
          .col(ColumnDef::new(Paste::Content).string_len(102400).not_null())
          .col(
            ColumnDef::new(Paste::Time)
              .timestamp_with_time_zone()
              .not_null(),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Paste::Table).to_owned())
      .await
  }
}

#[derive(Iden)]
enum Paste {
  Table,
  Token,
  Lang,
  Content,
  Time,
}
