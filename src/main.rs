mod controllers;
#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello]).mount(
        "/query",
        routes![controllers::mapcontroller::get_location_by_latlng],
    )
}
