#[macro_use] extern crate rocket;

use rocket::{Request, fs::TempFile, form::Form};
use rocket_dyn_templates::{Template, context};
use std::{fs, path::Path, process::Command};

// Define a struct to receive the form data
#[derive(FromForm)]
struct Upload<'r> {
    file: TempFile<'r>,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { field: "value" })
}

#[post("/uploadimg", data = "<form>")]
async fn upload_img(mut form: Form<Upload<'_>>) {
    // Access the uploaded file
    let file = &mut form.file;

    // Define save path
    let save_path = format!("./uploads/uploaded_file.bmp");

    // Get save dir, make it if it doesnt exist
    let save_dir = Path::new(&save_path).parent().unwrap();
    if !save_dir.exists()
    {
        let _ = fs::create_dir_all(save_dir);
    }

    // Save the file
    let _ = file.persist_to(save_path.clone()).await;

    // Get the file type
    let content_type_str = file.content_type().unwrap().to_string();
    let content_type = content_type_str.split('/').nth(1).unwrap();

    // If the file type is not .bmp, use imagemagick to convert it into bmp
    if content_type != "bmp"
    {
        println!("Not bmp");
        Command::new("convert").arg(save_path.clone()).arg(save_path.clone()).output().expect("Failed to execute");
    }
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oops! The path '{}' was not found.", req.uri())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, upload_img])
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}
