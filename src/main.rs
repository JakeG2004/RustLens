#[macro_use] extern crate rocket;

use rocket::{Request, fs::{ TempFile, FileServer, relative }, form::Form, data::{ Limits, ByteUnit }, Config, Rocket, Build };
use rocket_dyn_templates::{Template, context};

mod filters;

// Define a struct to receive the form data
#[derive(FromForm)]
struct Upload<'r> {
    file: TempFile<'r>,
    filter: String,
}

const SAVE_PATH: &str = "./uploads/uploaded_file.bmp";

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { field: "value" })
}

#[post("/uploadimg", data = "<form>")]
async fn upload_img(mut form: Form<Upload<'_>>) {
    let file = &mut form.file;

    // Persist file
    let _ = file.persist_to(SAVE_PATH).await;

    let selected_filter = form.filter.clone();

    // Convert to modifiable BMP
    filters::convert_to_bitmap(SAVE_PATH);

    // Get user selected filter
    match selected_filter.as_str()
    {
        "grayscale" => filters::apply_greyscale(SAVE_PATH.to_string()),
        "invert" => filters::apply_negative(SAVE_PATH.to_string()),
        "edge-detect" => filters::edge_detect(SAVE_PATH.to_string()),
        "blur" => filters::apply_blur(SAVE_PATH.to_string()),
        "sepia" => filters::apply_sepia(SAVE_PATH.to_string()),
        "posterize" => filters::apply_posterize(SAVE_PATH.to_string()),
        "popify" => filters::apply_popify(SAVE_PATH.to_string()),
        "pixelate" => filters::apply_pixelize(SAVE_PATH.to_string()),
        "flip-x" => filters::flip_x(SAVE_PATH.to_string()),
        "flip-y" => filters::flip_y(SAVE_PATH.to_string()),
        _ => println!("No"),
    }
}


#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oops! The path '{}' was not found.", req.uri())
}

#[launch]
fn rocket() -> Rocket<Build> {
    // Modify file upload limits so that it can filter its own image
    let limits = Limits::default()
        .limit("file", ByteUnit::Megabyte(10))
        .limit("data-form", ByteUnit::Megabyte(10));

    let config = Config::figment()
        .merge(("limits", limits));

    rocket::custom(config)
        .mount("/", routes![index, upload_img])
        .mount("/uploads", FileServer::from(relative!("uploads")))
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}
