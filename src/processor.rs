use image::{DynamicImage, Rgba, RgbaImage};
use crate::cli::{Dither};

const BT709 :[f32; 3] = [0.2126, 0.7152, 0.0722]; // see ITU-R BT.709

const BAYER8: [[u8; 8]; 8] =
    [[0, 32, 8, 40, 2, 34, 10, 42],
    [48, 16, 56, 24, 50, 18, 58, 26],
    [12, 44, 4, 36, 14, 46, 6, 38],
    [60, 28, 52, 20, 62, 30, 54, 22],
    [3, 35, 11, 43, 1, 33, 9, 41],
    [51, 19, 59, 27, 49, 17, 57, 25],
    [15, 47, 7, 39, 13, 45, 5, 37],
    [63, 31, 55, 23, 61, 29, 53, 21]];

pub fn process_gray(mut buf: RgbaImage) -> DynamicImage {
    for pix in buf.pixels_mut() {
        let r = pix[0] as f32 * BT709[0];
        let g = pix[1] as f32 * BT709[1];
        let b = pix[2] as f32 * BT709[2];
        let average = (r + g + b) as u8;
        *pix = Rgba([average, average, average, pix.0[3]]);
    }
    DynamicImage::ImageRgba8(buf)
}

pub fn process_image(mut buf: RgbaImage, palette: Vec<[u8; 3]>, dither: Dither) -> DynamicImage {
    for (x, y, pix) in buf.enumerate_pixels_mut() {
        let dither_shift = match dither {
            Dither::None => 0.0,
            Dither::Bayer8 => BAYER8[(y % 8) as usize][(x % 8) as usize] as f32 - 31.5,
        };

        let r = (pix[0] as f32 + dither_shift).clamp(0.0, 255.0);
        let g = (pix[1] as f32 + dither_shift).clamp(0.0, 255.0);
        let b = (pix[2] as f32 + dither_shift).clamp(0.0, 255.0);

        let mut min_diff = f32::MAX;
        let mut best = palette[0];
        for pal in &palette {
            let dr = r - pal[0] as f32;
            let dg = g - pal[1] as f32;
            let db = b - pal[2] as f32;
            let diff = dr * dr + dg * dg + db * db;

            if diff < min_diff {
                best = *pal;
                min_diff = diff;

            }
        }
        *pix = Rgba([best[0], best[1], best[2], 0xFF]);
    }
    DynamicImage::ImageRgba8(buf)
}

pub fn get_palette(str: String) -> Vec<[u8; 3]> {
    let content = std::fs::read_to_string(str).expect("Should have been able to read file");

    let mut palette = Vec::new();
    for l in content.lines() {
        if l.starts_with("//") || l.is_empty() { continue }
        let r = u8::from_str_radix(&l[0..2], 16).expect("Should have been able to parse red");
        let g = u8::from_str_radix(&l[2..4], 16).expect("Should have been able to parse green");
        let b = u8::from_str_radix(&l[4..6], 16).expect("Should have been able to parse blue");
        palette.push([r, g, b]);
    }
    palette
}