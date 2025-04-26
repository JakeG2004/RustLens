use rust_lens::filters;

const SAVE_PATH: &str = "./uploads/uploaded_file.bmp";

fn main()
{
    filters::apply_pixelize(SAVE_PATH.to_string());
}