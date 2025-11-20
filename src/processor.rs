use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

pub const BT709 :[f32; 3] = [0.2126, 0.7152, 0.0722]; // see ITU-R BT.709

pub fn process_gray(mut img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let pix = img.get_pixel(x, y).0;
            let r = pix[0] as f32 * BT709[0];
            let g = pix[1] as f32 * BT709[1];
            let b = pix[2] as f32 * BT709[2];
            let average = (r + g + b) as u8;
            img.put_pixel(x, y, Rgba([average, average, average, 0xFF]));
        }
    }
    img
}

pub fn process_image(mut img: DynamicImage, palette: &[[u8; 3]]) -> DynamicImage {
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let pix = img.get_pixel(x, y);

            let mut min_diff = f32::MAX;
            let mut best = palette[0];
            for pal in palette {
                let dr = pix[0] as f32 - pal[0] as f32;
                let dg = pix[1] as f32 - pal[1] as f32;
                let db = pix[2] as f32 - pal[2] as f32;
                let diff = dr * dr + dg * dg + db * db;

                if diff < min_diff {
                    best = *pal;
                    min_diff = diff;
                }
            }
            img.put_pixel(x, y, Rgba([best[0], best[1], best[2], 0xFF]));
        }
    }
    img
}