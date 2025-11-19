use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use crate::palettes::GRAY8;

pub fn process_gray8(img: &mut DynamicImage) -> DynamicImage {
    println!("\x1b[32;1mConverting to 8Bit grayscale \x1b[0m\n");
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let px = img.get_pixel(x, y).0;
            let r = px[0];
            let g = px[1];
            let b = px[2];

            let average = r/3 + g/3 + b/3;
            let mut best = GRAY8[0];
            let mut min_diff = u8::MAX;

            for p in GRAY8 {
                let mut diff :u8 = 0;
                if p > average {
                    diff = p - average;
                } else {
                    diff = average - p;
                }
                if diff < min_diff {
                    min_diff = diff;
                    best = p;
                }
            }
            img.put_pixel(x, y, Rgba([best, best, best, 0xFF]));
        }
    }
    img.clone()
}

pub fn process_gray(img: &mut DynamicImage) -> DynamicImage {
    println!("\x1b[32;1mConverting to grayscale \x1b[0m\n");
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let px = img.get_pixel(x, y).0;
            let r = px[0];
            let g = px[1];
            let b = px[2];

            let average = r/3 + g/3 + b/3;
            img.put_pixel(x, y, Rgba([average, average, average, 0xFF]));
        }
    }
    img.clone()
}

pub fn process_gameboy(img: &image::DynamicImage) -> DynamicImage {
    println!("processing gameboy\n");
    img.grayscale()
}