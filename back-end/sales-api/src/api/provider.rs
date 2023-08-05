use async_trait::async_trait;
use sqlx::{MySqlConnection, Result};

#[async_trait]
pub trait Database {
    async fn connect() -> Result<MySqlConnection>;
    async fn get_db_url(&self) -> Result<String>;
}
