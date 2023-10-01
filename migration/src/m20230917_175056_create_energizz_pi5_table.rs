use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(DadosSensor1::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DadosSensor1::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DadosSensor1::Temperatura).float().not_null())
                    .col(ColumnDef::new(DadosSensor1::Umidade).float().not_null())
                    .col(ColumnDef::new(DadosSensor1::Pressao).float().not_null())
                    .col(ColumnDef::new(DadosSensor1::PacienteId).text().not_null())
                    .col(ColumnDef::new(DadosSensor1::PostingTime).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(DadosSensor1::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum DadosSensor1 {
    Table,
    Id,
    Temperatura,
    Umidade,
    Pressao,
    PacienteId,
    PostingTime,
}
