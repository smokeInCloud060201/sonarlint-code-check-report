use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct CreateTables;

impl CreateTables {
    pub fn name() -> &'static str {
        "m20241201_000001_create_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreateTables {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create api_projects table (to avoid conflict with SonarQube's projects table)
        manager
            .create_table(
                Table::create()
                    .table(ApiProjects::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ApiProjects::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ApiProjects::SonarqubeKey).string().not_null().unique_key())
                    .col(ColumnDef::new(ApiProjects::Name).string().not_null())
                    .col(ColumnDef::new(ApiProjects::Visibility).string().not_null())
                    .col(ColumnDef::new(ApiProjects::Qualifier).string().not_null())
                    .col(
                        ColumnDef::new(ApiProjects::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(ApiProjects::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(ApiProjects::SonarqubeCreatedAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(ApiProjects::Description).text())
                    .col(ColumnDef::new(ApiProjects::Language).string())
                    .col(ColumnDef::new(ApiProjects::Tags).text())
                    .col(
                        ColumnDef::new(ApiProjects::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes for api_projects table
        manager
            .create_index(
                Index::create()
                    .name("idx_api_projects_sonarqube_key")
                    .table(ApiProjects::Table)
                    .col(ApiProjects::SonarqubeKey)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_api_projects_name")
                    .table(ApiProjects::Table)
                    .col(ApiProjects::Name)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_api_projects_is_active")
                    .table(ApiProjects::Table)
                    .col(ApiProjects::IsActive)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_api_projects_created_at")
                    .table(ApiProjects::Table)
                    .col(ApiProjects::CreatedAt)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        // Create sonarqube_tokens table
        manager
            .create_table(
                Table::create()
                    .table(SonarqubeTokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SonarqubeTokens::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SonarqubeTokens::Name).string().not_null())
                    .col(ColumnDef::new(SonarqubeTokens::Token).string().not_null().unique_key())
                    .col(ColumnDef::new(SonarqubeTokens::ProjectKey).string())
                    .col(
                        ColumnDef::new(SonarqubeTokens::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(SonarqubeTokens::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(SonarqubeTokens::ExpiresAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(SonarqubeTokens::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(SonarqubeTokens::CreatedBy).string())
                    .col(ColumnDef::new(SonarqubeTokens::Description).text())
                    .to_owned(),
            )
            .await?;

        // Create indexes for sonarqube_tokens table
        manager
            .create_index(
                Index::create()
                    .name("idx_sonarqube_tokens_name")
                    .table(SonarqubeTokens::Table)
                    .col(SonarqubeTokens::Name)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sonarqube_tokens_project_key")
                    .table(SonarqubeTokens::Table)
                    .col(SonarqubeTokens::ProjectKey)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sonarqube_tokens_is_active")
                    .table(SonarqubeTokens::Table)
                    .col(SonarqubeTokens::IsActive)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sonarqube_tokens_created_at")
                    .table(SonarqubeTokens::Table)
                    .col(SonarqubeTokens::CreatedAt)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_sonarqube_tokens_created_by")
                    .table(SonarqubeTokens::Table)
                    .col(SonarqubeTokens::CreatedBy)
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SonarqubeTokens::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ApiProjects::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ApiProjects {
    Table,
    Id,
    SonarqubeKey,
    Name,
    Visibility,
    Qualifier,
    CreatedAt,
    UpdatedAt,
    SonarqubeCreatedAt,
    Description,
    Language,
    Tags,
    IsActive,
}

#[derive(DeriveIden)]
enum SonarqubeTokens {
    Table,
    Id,
    Name,
    Token,
    ProjectKey,
    CreatedAt,
    UpdatedAt,
    ExpiresAt,
    IsActive,
    CreatedBy,
    Description,
}
