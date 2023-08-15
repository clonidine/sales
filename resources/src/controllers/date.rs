use chrono::Datelike;
use rocket::{
    http::Status,
    serde::json::{serde_json::json, Value},
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: i32,
}

impl Date {
    pub fn new(day: u32, month: u32, year: i32) -> Self {
        Date { day, month, year }
    }
}

#[rocket::get("/api/date")]
pub async fn current_date() -> Result<Value, Status> {
    let current_utc = chrono::Utc::now();

    let (day, month, year) = (current_utc.day(), current_utc.month(), current_utc.year());

    let current_date = Date::new(day, month, year);

    Ok(json!(current_date))
}
