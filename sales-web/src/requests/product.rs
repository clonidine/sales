use rocket::{
    http::Status,
    serde::json::{serde_json::json, Value},
};
use sales_api::api::repository::Repository;
use sales_impl::repository::product::repository::ProductRepository;

#[get("/products")]
pub async fn get_products() -> Result<Value, Status> {
    let products = ProductRepository::find_all().await;

    match products {
        Ok(products) => Ok(json!(products)),

        Err(_) => Err(Status::NotFound),
    }
}

#[get("/products/<id>")]
pub async fn get_product(id: u64) -> Result<Value, Status> {
    let product = ProductRepository::find_one(id).await;

    match product {
        Ok(product) => Ok(json!(product)),
        Err(_) => Err(Status::NotFound),
    }
}
