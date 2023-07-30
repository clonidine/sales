use dotenv::dotenv;
use mysql::{Opts, Pool, PooledConn};
use sales_api::api::provider::Database;

pub struct MySQL {}

impl Database for MySQL {
    fn get_db_url() -> Result<String, String> {
        let loaded_variables = dotenv().ok();

        if let Some(_) = loaded_variables {
            Ok(std::env::var("DB_URL").expect("Error while trying to get DB_URL variable"))
        } else {
            panic!("Error while trying to load environment variables.")
        }
    }

    fn connect(&self) -> Result<PooledConn, String> {
        let db_url = Self::get_db_url();

        match db_url {
            Ok(db_url) => {
                let opts = Opts::from_url(&db_url).expect("Invalid DB URL");

                let pool = Pool::new(opts);

                match pool {
                    Ok(pool) => {
                        let conn = pool.get_conn();

                        match conn {
                            Ok(conn) => Ok(conn),
                            Err(e) => panic!("{}", e),
                        }
                    }

                    Err(e) => panic!("Error while trying to create Pool: {}", e),
                }
            }

            Err(e) => panic!("Error while trying to get DB URL: {}", e),
        }
    }
}
