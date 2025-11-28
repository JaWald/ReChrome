use image::{DynamicImage, Rgba, RgbaImage};
use rayon::prelude::*;
use crate::cli::{Dither};
use crate::cli::Dither::*;
use crate::data;

pub fn process_image(mut buf: RgbaImage, palette: Vec<[u8; 3]>, dither: Dither, amplitude: f32) -> DynamicImage {
    buf.par_enumerate_pixels_mut().for_each(|(x, y, pix)| {
        let dither_shift = match dither {
            Raw => 0.0,
            Bayer2 => (data::BAYER2[(y % 2) as usize][(x % 2) as usize]) * amplitude,
            Bayer4 => (data::BAYER4[(y % 4) as usize][(x % 4) as usize]) * amplitude,
            Bayer8 => (data::BAYER8[(y % 8) as usize][(x % 8) as usize]) * amplitude,
            Bayer16 => (data::BAYER16[(y % 16) as usize][(x % 16) as usize]) * amplitude
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