pub mod entities;
pub mod service;
pub mod admin_token_entity;

use sea_orm::Database;
use sea_orm::DatabaseConnection;
use std::env;

pub async fn connect() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://sonar:sonar@localhost:5432/sonarcute".to_string());
    
    Database::connect(&database_url).await
}
