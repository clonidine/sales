use async_trait::async_trait;

#[async_trait]
pub trait Repository<T> {
    async fn save(obj: &T) -> Result<(), String>;
    async fn delete(id: usize) -> Result<(), String>;
    async fn find_one(id: usize) -> Result<T, String>;
    async fn create_table() -> Result<(), String>;
}
