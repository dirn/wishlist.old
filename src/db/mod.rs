use crate::config::Config;
use sea_orm::{Database, DatabaseConnection, DbErr};

#[allow(dead_code)]
pub async fn connect(config: &Config) -> Result<DatabaseConnection, DbErr> {
    Database::connect(&config.database.url).await
}
