use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct AddProjectFolderPath;

#[async_trait::async_trait]
impl MigrationTrait for AddProjectFolderPath {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ApiProjects::Table)
                    .add_column(
                        ColumnDef::new(ApiProjects::ProjectFolderPath)
                            .string()
                            .null()
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ApiProjects::Table)
                    .drop_column(ApiProjects::ProjectFolderPath)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ApiProjects {
    Table,
    ProjectFolderPath,
}
