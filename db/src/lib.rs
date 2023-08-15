pub mod databases;

use async_trait::async_trait;
use sqlx::Connection;

#[async_trait]
pub trait Database {
    type DbConnection: Connection;

    async fn connect() -> Result<Self::DbConnection, String>;
    async fn collect_db_url() -> Result<String, String>;
}
