use super::mysql::MySQL;

pub struct DBAdapter {}

impl DBAdapter {
    pub fn new() -> MySQL {
        MySQL {}
    }
}
