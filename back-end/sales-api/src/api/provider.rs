use mysql::PooledConn;

pub trait Database {
    fn connect(&self) -> Result<PooledConn, String>;
    fn get_db_url() -> Result<String, String>;
}
