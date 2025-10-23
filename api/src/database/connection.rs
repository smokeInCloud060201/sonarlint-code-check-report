use sea_orm::{Database, DatabaseConnection as SeaDatabaseConnection, DbErr, ConnectionTrait};
use tracing::info;

use crate::database::config::DatabaseConfig;

pub type DatabaseConnection = SeaDatabaseConnection;

pub async fn create_connection(config: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    let connection_url = config.connection_url();
    info!("Connecting to database: {}", connection_url.replace(&config.password, "***"));

    let db = Database::connect(&connection_url).await?;
    
    info!("Successfully connected to PostgreSQL database");
    Ok(db)
}

pub async fn test_connection(db: &DatabaseConnection) -> Result<(), DbErr> {
    // Test the connection by running a simple query
    let result = db.execute_unprepared("SELECT 1").await?;
    info!("Database connection test successful: {} rows returned", result.rows_affected());
    Ok(())
}
