use domain::product::Product;
use repository::{product::ProductRepositoryMySQL, ProductRepositoryAbstract};
use rocket::{
    http::Status,
    serde::json::{serde_json::json, Json, Value},
};

#[rocket::get("/api/products")]
pub async fn find_all() -> Result<Value, Status> {
    let products = ProductRepositoryMySQL::find_all().await;

    match products {
        Ok(products) => Ok(json!(products)),
        Err(_) => Err(Status::NotFound),
    }
}

#[rocket::get("/api/products/<id>")]
pub async fn find_one(id: u64) -> Result<Value, Status> {
    let product = ProductRepositoryMySQL::find_one(id).await;

    match product {
        Ok(product) => Ok(json!(product)),
        Err(_) => Err(Status::NotFound),
    }
}

#[rocket::post("/api/products", format = "json", data = "<product>")]
pub async fn save(product: Json<Product>) -> Status {
    let product_saved = ProductRepositoryMySQL::save(&product).await;

    match product_saved {
        Ok(_) => Status::Created,

        Err(_) => Status::Unauthorized,
    }
}

#[rocket::delete("/api/products/<id>")]
pub async fn delete(id: u64) -> Status {
    let deleted_product = ProductRepositoryMySQL::delete(id).await;

    match deleted_product {
        Ok(_) => Status::Ok,
        Err(_) => Status::NotFound,
    }
}
