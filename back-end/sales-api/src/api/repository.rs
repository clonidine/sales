pub trait Repository {
    fn save(&self) -> Result<(), String>;
    fn delete(&self) -> Result<(), String>;
    fn find_one(&self, id: usize) -> Option<&Self>;
    fn create_table(&self) -> Result<(), String>;
    fn get_table_name() -> &'static str;
}
