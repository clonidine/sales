use chrono::Datelike;
use rocket::serde::json::Json;
use sales_api::api::domain::date::Date;

#[get("/current-date")]
pub async fn current_date() -> Json<Date> {
    let current_utc = chrono::Utc::now();

    let (day, month, year) = (current_utc.day(), current_utc.month(), current_utc.year());

    let current_date = Date::new(day, month, year);

    Json(current_date)
}
