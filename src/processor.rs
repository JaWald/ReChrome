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

    if dither != Fs {
        buf.par_enumerate_pixels_mut().for_each(|(x, y, pix)| {
            let dither_shift = match dither {
                Raw => 0.0,
                Bayer2  => (data::BAYER2[(y % 2) as usize][(x % 2) as usize]) * amplitude,
                Bayer4  => (data::BAYER4[(y % 4) as usize][(x % 4) as usize]) * amplitude,
                Bayer8  => (data::BAYER8[(y % 8) as usize][(x % 8) as usize]) * amplitude,
                Bayer16 => (data::BAYER16[(y % 16) as usize][(x % 16) as usize]) * amplitude,
                Fs => 0.0,
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
    } else {
        let height = buf.height();
        let width = buf.width() * 4;
        let right_factor = 7f32/16f32;
        let down_middle_factor = 5f32/16f32;
        let down_left_factor = 3f32/16f32;
        let down_right_factor = 1f32/16f32;
        let mut image = buf.to_vec().into_iter().map(|color| color as f32).collect::<Vec<f32>>();

        for y in 0..height {
            for x in (0..width).step_by(4) {
                let index = (y * width + x) as usize;

                let r_old = image[index + 0];
                let g_old = image[index + 1];
                let b_old = image[index + 2];
                let a_old = image[index + 3];

                let r_clamped = r_old.clamp(0.0, 255.0);
                let g_clamped = g_old.clamp(0.0, 255.0);
                let b_clamped = b_old.clamp(0.0, 255.0);

                let nearest = tree.nearest_one::<SquaredEuclidean>(&[r_clamped, g_clamped, b_clamped]);
                let color = palette[nearest.item as usize];
                buf.put_pixel(x / 4, y, Rgba([color[0] as u8, color[1] as u8, color[2] as u8, a_old as u8]));

                let diff = [
                    (r_old - color[0]).clamp(-amplitude, amplitude),
                    (g_old - color[1]).clamp(-amplitude, amplitude),
                    (b_old - color[2]).clamp(-amplitude, amplitude)
                ];

                if x < (width - 4) {
                    // right
                    let r_loc = (y * width + (x + 4) + 0) as usize;
                    image[r_loc + 0] += diff[0] * right_factor;
                    image[r_loc + 1] += diff[1] * right_factor;
                    image[r_loc + 2] += diff[2] * right_factor;
                }
                if y < (height - 1) {
                    // down middle
                    let r_loc = ((y + 1) * width + x + 0) as usize;
                    image[r_loc + 0] += diff[0] * down_middle_factor;
                    image[r_loc + 1] += diff[1] * down_middle_factor;
                    image[r_loc + 2] += diff[2] * down_middle_factor;
                    // down left
                    if x > 0 {
                        let r_loc = ((y + 1) * width + (x - 4) + 0) as usize;
                        image[r_loc + 0] += diff[0] * down_left_factor;
                        image[r_loc + 1] += diff[1] * down_left_factor;
                        image[r_loc + 2] += diff[2] * down_left_factor;
                    }
                    // down right
                    if x < (width - 4) {
                        let r_loc = ((y + 1) * width + (x + 4) + 0) as usize;
                        image[r_loc + 0] += diff[0] * down_right_factor;
                        image[r_loc + 1] += diff[1] * down_right_factor;
                        image[r_loc + 2] += diff[2] * down_right_factor;
                    }
                }
            }
        }
    }
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