use rust_lens::filters;

const SAVE_PATH: &str = "./uploads/uploaded_file.bmp";

fn main()
{
    filters::flip_y(SAVE_PATH.to_string());
}