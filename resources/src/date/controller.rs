use chrono::Datelike;
use rocket::{
    http::Status,
    serde::json::{serde_json::json, Value},
};

use super::Date;

#[rocket::get("/api/date")]
pub async fn current_date() -> Result<Value, Status> {
    let current_utc = chrono::Utc::now();

    let (day, month, year) = (current_utc.day(), current_utc.month(), current_utc.year());

    let current_date = Date::new(day, month, year);

    Ok(json!(current_date))
}
