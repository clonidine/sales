use async_trait::async_trait;
use sqlx::{AnyConnection, Result};

#[async_trait]
pub trait Database {
    async fn connect() -> Result<AnyConnection>;
    async fn get_db_url() -> Result<String>;
}
