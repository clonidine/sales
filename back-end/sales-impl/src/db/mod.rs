pub mod implementations;
pub mod mysql;

#[cfg(test)]
mod db_tests {
    use sales_api::api::provider::Database;

    use crate::db::implementations::DBImplementation;

    #[test]
    fn opening_conn() {
        let db = DBImplementation::new();

        let conn = db.connect();

        assert!(conn.is_ok())
    }
}
