use mysql::PooledConn;

pub trait Database {
    fn connect() -> Result<PooledConn, String>;
    fn get_db_url() -> Result<String, String>;
}
