#[macro_use]
extern crate error_chain;
extern crate image;
extern crate itertools;
extern crate term;

use image::{GenericImage, Pixel};
use itertools::Itertools;

mod errors;

use errors::*;

const HISTOGRAM_HEIGHT: i32 = 16;

pub fn run(filename: &str) -> Result<()> {
    let img = image::open(filename).chain_err(|| "failed to open image")?;
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
    let max = red_histogram
        .iter()
        .chain(green_histogram.iter())
        .chain(blue_histogram.iter())
        .max()
        .ok_or("failed to find max value")?;
    plot_histogram(&red_histogram, *max, term::color::BRIGHT_RED)?;
    plot_histogram(&green_histogram, *max, term::color::BRIGHT_GREEN)?;
    plot_histogram(&blue_histogram, *max, term::color::BRIGHT_BLUE)?;
    Ok(())
}

fn plot_histogram(histogram: &[i32], max: i32, color: term::color::Color) -> Result<()> {
    // resample histogram from 256 to 128 bins to fit on the screen
    let histogram: Vec<i32> = histogram
        .iter()
        .chunks(2)
        .into_iter()
        .map(|chunk| chunk.sum())
        .collect();
    let bars: Vec<f32> = histogram
        .iter()
        .map(|x| (HISTOGRAM_HEIGHT * x) as f32 / (max as f32))
        .collect();
    let mut t = term::stdout().ok_or("failed to access stdout")?;
    t.fg(color).chain_err(|| "failed to set font color")?;
    for row in 0..HISTOGRAM_HEIGHT + 1 {
        let mut line = String::with_capacity(histogram.len());
        for bar in &bars {
            line.push(to_character(*bar, row));
        }
        println!("{}", line);
    }
    t.reset().chain_err(|| "failed to reset stdout")?;
    Ok(())
}

fn to_character(bar: f32, row: i32) -> char {
    let truncated_bar = bar as i32;
    let top = HISTOGRAM_HEIGHT - row;
    if truncated_bar > top {
        '█'
    } else if truncated_bar == top {
        let frac = bar.fract();
        if frac > 0.875 {
            '▇'
        } else if frac > 0.75 {
            '▆'
        } else if frac > 0.625 {
            '▅'
        } else if frac > 0.5 {
            '▄'
        } else if frac > 0.375 {
            '▃'
        } else if frac > 0.25 {
            '▂'
        } else if frac > 0.125 {
            '▁'
        } else {
            ' '
        }

    } else {
        ' '
    }
}
