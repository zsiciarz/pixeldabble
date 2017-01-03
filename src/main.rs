extern crate image;

use image::{GenericImage, ImageError, Pixel};

fn run(filename: &str) -> Result<(), ImageError> {
    let img = image::open(filename)?;
    let (width, height) = img.dimensions();
    println!("{}: {}x{} px", filename, width, height);
    let mut red_histogram = vec![0; 256];
    for (_, _, pixel) in img.pixels() {
        let (r, _, _, _) = pixel.channels4();
        red_histogram[r as usize] += 1;
    }
    println!("{:?}", red_histogram);
    Ok(())
}

fn main() {
    let image_file = std::env::args().nth(1).unwrap_or_else(|| {
        println!("USAGE: pixeldabble <FILENAME>");
        std::process::exit(1);
    });
    if let Err(ref e) = run(&image_file[..]) {
        println!("error: {}", e);
        std::process::exit(1);
    }
}
