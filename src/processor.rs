use image::{DynamicImage, Rgba, RgbaImage};
use rayon::prelude::*;
use crate::cli::{Dither};
use crate::cli::Dither::*;
use crate::data;

pub fn process_image(mut buf: RgbaImage, palette: Vec<[u8; 3]>, dither: Dither, amplitude: f32) -> DynamicImage {
    buf.par_enumerate_pixels_mut().for_each(|(x, y, pix)| {
        let dither_shift = match dither {
            Raw => 0.0,
            Bayer2 => (data::BAYER2[(y % 2) as usize][(x % 2) as usize] as f32 / 4.0 - 0.5) * amplitude,
            Bayer4 => (data::BAYER4[(y % 4) as usize][(x % 4) as usize] as f32 / 16.0 - 0.5) * amplitude,
            Bayer8 => (data::BAYER8[(y % 8) as usize][(x % 8) as usize] as f32 / 64.0 - 0.5) * amplitude,
            Bayer16 => (data::BAYER16[(y % 16) as usize][(x % 16) as usize] as f32 / 256.0 - 0.5) * amplitude
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
    });
    DynamicImage::ImageRgba8(buf)
}

/* for palette conversion in development

pub fn print_palette(str: &String) {
    let content = std::fs::read_to_string(str).expect("Should have been able to read file");

    let mut count = 0;
    for l in content.lines() {
        if l.starts_with("//") || l.is_empty() { continue }
        let r = u8::from_str_radix(&l[0..2], 16).expect("Should have been able to parse red");
        let g = u8::from_str_radix(&l[2..4], 16).expect("Should have been able to parse green");
        let b = u8::from_str_radix(&l[4..6], 16).expect("Should have been able to parse blue");
        print!("[{:>3}, {:>3}, {:>3}], ", r, g, b);
        if count % 4 == 3 {
            println!();
        }
        count += 1;
    }
    println!();
}
*/