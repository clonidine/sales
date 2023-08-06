mod response;
mod index;
mod models;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![index::index, response::hello, response::hey])
}
