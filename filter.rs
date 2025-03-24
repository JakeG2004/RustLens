use std::env;
use std::fs::File;

fn main()
{
    // Get arguments
    let args: Vec<String> = env::args().collect();
   
    // Ensure proper argument usage
    if args.len() != 2
    {
        println!("ERROR: Proper usage ./filter <file.bmp>");
        std::process::exit(-1);
    }

    let filename = &args[1];

    // Ensure that the file exists
    let mut file = File::open(filename);

    dbg!(file);
}