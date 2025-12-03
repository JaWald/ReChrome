use image::{DynamicImage, Rgba, RgbaImage};
use turbojpeg;
use rayon::prelude::*;
use kiddo::{ImmutableKdTree, SquaredEuclidean};
use crate::cli::{Dither, Format};
use crate::cli::Dither::*;
use crate::data;
use crate::error::AppError;

pub fn process_image(mut buf: RgbaImage, palette: Vec<[f32; 3]>, dither: Dither, amplitude: f32) -> DynamicImage {
    let tree: ImmutableKdTree<f32, 3> = ImmutableKdTree::new_from_slice(&*palette);

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
        let pixel_dithered = [r, g, b];

        let nearest = tree.nearest_one::<SquaredEuclidean>(&pixel_dithered);
        let index = nearest.item as usize;
        let color = palette[index];

        *pix = Rgba([color[0] as u8, color[1] as u8, color[2] as u8, pix[3]]);
    });
    DynamicImage::ImageRgba8(buf)
}

pub fn save_image(processed: &DynamicImage, output: &String, format: &Format, quality: u8) -> Result<(), AppError> {
    match format {
        Format::Png =>  processed.save(output)?,
        Format::Jpeg => {
            let vec = processed.to_rgb8().to_vec();
            let pixels = vec.as_slice();
            let image = turbojpeg::Image {
                pixels,
                width: processed.width() as usize,
                pitch: processed.width() as usize * turbojpeg::PixelFormat::RGB.size(),
                height: processed.height() as usize,
                format: turbojpeg::PixelFormat::RGB
            };
            let mut compressor = turbojpeg::Compressor::new()?;
            compressor.set_quality(quality as i32)?;
            compressor.set_subsamp(turbojpeg::Subsamp::Sub2x2)?;
            let mut output_buf = turbojpeg::OutputBuf::new_owned();
            compressor.compress(image, &mut output_buf)?;
            std::fs::write(output, output_buf)?;
        }
    }
    Ok(())
}