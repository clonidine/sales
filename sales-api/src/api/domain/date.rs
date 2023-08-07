#[derive(serde::Deserialize, serde::Serialize)]
pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: i32,
}

impl Date {
    pub fn new(day: u32, month: u32, year: i32) -> Date {
        Date { day, month, year }
    }
}