use rocket::{
    http::Status,
    serde::json::{serde_json::json, Value},
};

#[get("/hello")]
pub async fn hello() -> Result<Value, Status> {
    Ok(json!("Hello!!!!"))
}

#[get("/numbers")]
pub async fn hey() -> Result<Value, Status> {
    let numbers = vec![1, 2, 3, 4, 5];

    Ok(json!(numbers))
}
