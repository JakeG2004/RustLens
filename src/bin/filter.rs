use std::{env, process};

mod filters;

fn main()
{
    // Get CLI arguments
    let args: Vec<String> = env::args().collect();

    let img = bmp::open("./uploads/uploaded_file.bmp").unwrap_or_else(|e| 
    {
        panic!("Failed to open: {}", e);
    });

    if args.len() != 2
    {
        println!("Invalid usage");
        process::exit(-1);
    }

    match args.get(1).map(|s| s.as_str()) {
        Some("greyscale") => {
            filters::apply_greyscale(img);
        }
        Some(other) => {
            println!("Unknown argument: '{}'", other);
            process::exit(-1);
        }
        None => {
            println!("No filter argument provided.");
            process::exit(-1);
        }
    }
}