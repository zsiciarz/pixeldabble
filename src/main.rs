extern crate image;

use image::{GenericImage, ImageError, Pixel};

const HISTOGRAM_HEIGHT: i32 = 16;

fn run(filename: &str) -> Result<(), ImageError> {
    let img = image::open(filename)?;
    let (width, height) = img.dimensions();
    println!("{}: {}x{} px", filename, width, height);
    let mut red_histogram = vec![0; 256];
    for (_, _, pixel) in img.pixels() {
        let (r, _, _, _) = pixel.channels4();
        red_histogram[r as usize] += 1;
    }
    plot_histogram(&red_histogram);
    Ok(())
}

fn plot_histogram(histogram: &[i32]) {
    println!("{:?}", histogram);
    let max = histogram.iter().max().unwrap();
    // TODO: resampling
    let bars: Vec<i32> = histogram.iter().take(128).map(|x| HISTOGRAM_HEIGHT * x / max).collect();
    for row in 0..HISTOGRAM_HEIGHT + 1 {
        let mut line = String::with_capacity(histogram.len());
        for bar in &bars {
            line.push(if *bar >= (HISTOGRAM_HEIGHT - row) {
                '_'
            } else {
                ' '
            });
        }
        println!("{}", line);
    }
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
