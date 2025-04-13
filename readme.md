# RustLens
## What is RustLens?
RustLens is an image filtering suite built for the web with Rust on the backend. It allows users to choose from a number of filtering options with their uploaded images. Currently supports `.bmp`, `.jpg`, and `.jpeg` image formats.

## How do I use it?
There are two options here:
1) Visit my website! (Preferred)
    * Once the website is ready, I will link it here. Stay excited!
2) Run it yourself
    1) Ensure [ImageMagick](https://imagemagick.org/) is installed.
    2) Download all of the files from this repo
        * Either click the green `<> Code` button, then select `Download ZIP`
        * Or run the following command: `git clone https://github.com/jakeg2004/RustLens.git`
    3) Navigate to the RustLens directory
    4) In your terminal, type `cargo run` to automatically compile and run.
    5) Click on the link that will provided in your terminal (default is `127.0.0.1:8000`).

## Dependencies
* [Rocket Web Framework for Rust](https://rocket.rs/)
* [BMP crate for Rust](https://docs.rs/bmp/latest/bmp/)
* [ImageMagick](https://imagemagick.org/)
