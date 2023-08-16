pub mod controller;

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
