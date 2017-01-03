extern crate image;
extern crate itertools;
extern crate term;

use image::{GenericImage, ImageError, Pixel};
use itertools::Itertools;

const HISTOGRAM_HEIGHT: i32 = 16;

pub fn run(filename: &str) -> Result<(), ImageError> {
    let img = image::open(filename)?;
    let (width, height) = img.dimensions();
    println!("{}: {}x{} px", filename, width, height);
    let mut red_histogram = vec![0; 256];
    let mut green_histogram = vec![0; 256];
    let mut blue_histogram = vec![0; 256];
    for (_, _, pixel) in img.pixels() {
        let (r, g, b, _) = pixel.channels4();
        red_histogram[r as usize] += 1;
        green_histogram[g as usize] += 1;
        blue_histogram[b as usize] += 1;
    }
    let max = red_histogram.iter()
        .chain(green_histogram.iter())
        .chain(blue_histogram.iter())
        .max()
        .unwrap();
    plot_histogram(&red_histogram, *max, term::color::BRIGHT_RED);
    plot_histogram(&green_histogram, *max, term::color::BRIGHT_GREEN);
    plot_histogram(&blue_histogram, *max, term::color::BRIGHT_BLUE);
    Ok(())
}

fn plot_histogram(histogram: &[i32], max: i32, color: term::color::Color) {
    // resample histogram from 256 to 128 bins to fit on the screen
    let histogram: Vec<i32> =
        histogram.iter().chunks(2).into_iter().map(|chunk| chunk.sum()).collect();
    let bars: Vec<i32> = histogram.iter().map(|x| HISTOGRAM_HEIGHT * x / max).collect();
    let mut t = term::stdout().unwrap();
    t.fg(color).unwrap();
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
    t.reset().unwrap();
}