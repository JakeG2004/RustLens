#[macro_use] extern crate rocket;
use rocket::Request;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    return Template::render("index", context! { field: "value" });
}

#[catch(404)]
fn not_found(req: &Request) -> String
{
    return format!("Oops! The path '{}' was not found.", req.uri());
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/", catchers![not_found])
        .attach(Template::fairing())

}
