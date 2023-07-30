use super::mysql::MySQL;

pub struct DBImplementation {}

impl DBImplementation {
    pub fn new() -> MySQL {
        MySQL {}
    }
}
