use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use crate::palettes::GRAY8;

/* TODO:
    - Create variable amount of color bands
    - Increase performance by ordering grayscale colors by "size",
        only search until palette value bigger than average is found
        -> only have to search half the array on average
*/



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
                if p > average {
                    if (p - average) < min_diff {
                        min_diff = p - average;
                        best = p;
                    }
                } else {
                    if (average - p) < min_diff {
                        min_diff = average - p;
                        best = p;
                    }
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