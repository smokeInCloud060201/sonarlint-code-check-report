use sea_orm_migration::prelude::*;

mod m20241201_000001_create_tables;
mod m20241201_000002_add_project_folder_path;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241201_000001_create_tables::CreateTables),
            Box::new(m20241201_000002_add_project_folder_path::AddProjectFolderPath),
        ]
    }
}

#[tokio::main]
async fn main() {
    cli::run_cli(Migrator).await;
}
