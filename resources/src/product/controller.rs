use domain::product::Product;
use repository::{product::ProductRepositoryMySQL, ProductRepositoryAbstract};
use rocket::{
    http::Status,
    serde::json::{serde_json::json, Json, Value},
};

use super::StockUpdate;

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

#[rocket::put("/api/products/<id>/stock", data = "<stock_update>")]
pub async fn update_stock(id: u64, stock_update: Json<StockUpdate>) -> Result<Value, Status> {
    let column_filter = "id";
    let column_update = "stock";

    let filter = id.to_string();
    let value = stock_update.stock.to_string();

    let updated =
        ProductRepositoryMySQL::update(column_filter, column_update, &filter, &value).await;

    match updated {
        Ok(_) => {
            let product_updated = ProductRepositoryMySQL::find_one(id).await;

            match product_updated {
                Ok(product) => Ok(json!(product)),
                Err(_) => Err(Status::NotFound),
            }
        }
        Err(_) => Err(Status::NotFound),
    }
}
