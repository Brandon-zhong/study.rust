#[macro_use]
extern crate rocket;

#[get("/index")]
fn index() -> &'static str {
    "hello world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/hello", routes![index])
}
