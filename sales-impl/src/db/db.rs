use std::env;

use async_trait::async_trait;
use dotenv::dotenv;
use sales_api::api::provider::Database;
use sqlx::{any::install_default_drivers, Connection, MySqlConnection, Result};

pub struct DatabaseImpl;

#[async_trait]
impl Database for DatabaseImpl {
    async fn connect() -> Result<MySqlConnection> {
        install_default_drivers();

        let db_url = Self::get_db_url().await?;

        Ok(MySqlConnection::connect(&db_url).await?)
    }

    async fn get_db_url() -> Result<String> {
        let loaded_var_environment = dotenv().ok();

        match loaded_var_environment {
            Some(_) => {
                let db_url = env::var("DB_URL");

                match db_url {
                    Ok(db_url) => Ok(db_url),
                    Err(e) => panic!("{}", e),
                }
            }

            None => panic!("Cannot load environment variables."),
        }
    }
}

#[cfg(test)]
mod db_tests {
    use sales_api::api::provider::Database;

    use super::DatabaseImpl;

    #[tokio::test]
    async fn connecting_ok() {
        let connection = DatabaseImpl::connect().await;

        match connection {
            Ok(_) => assert!(true),

            Err(e) => {
                println!("Error while getting connection: {}", e);
                assert!(false)
            }
        }
    }
}
