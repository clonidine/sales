use async_trait::async_trait;
use dotenv::dotenv;
use sqlx::{Connection, MySqlConnection};

use crate::Database;

pub struct DbMySql;

#[async_trait]
impl Database for DbMySql {
    type DbConnection = MySqlConnection;

    async fn connect() -> Result<Self::DbConnection, String> {
        let db_url = Self::collect_db_url().await?;

        let conn = MySqlConnection::connect(&db_url).await;

        match conn {
            Ok(conn) => Ok(conn),
            Err(e) => panic!("{}", e),
        }
    }

    async fn collect_db_url() -> Result<String, String> {
        let _ = dotenv()
            .map_err(|err| panic!("Error while trying to load environment variables: {}", err));

        let db_url = dotenv::var("DB_URL");

        match db_url {
            Ok(url) => Ok(url),
            Err(err) => panic!("DB_URL key not found: {}", err),
        }
    }
}

#[cfg(test)]
mod db_test {
    use crate::Database;

    use super::DbMySql;

    #[tokio::test]
    async fn connecting() {
        let conn = DbMySql::connect().await;

        assert!(conn.is_ok())
    }
}
