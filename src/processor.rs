use image::{DynamicImage, Rgba, RgbaImage};

pub const BT709 :[f32; 3] = [0.2126, 0.7152, 0.0722]; // see ITU-R BT.709

pub fn process_gray(mut buf: RgbaImage) -> DynamicImage {
    for pix in buf.pixels_mut() {
        let r = pix[0] as f32 * BT709[0];
        let g = pix[1] as f32 * BT709[1];
        let b = pix[2] as f32 * BT709[2];
        let average = (r + g + b) as u8;
        *pix = Rgba([average, average, average, 0xFF]);
    }
    DynamicImage::ImageRgba8(buf)
}

pub fn process_image(mut buf: RgbaImage, palette: &[[u8; 3]]) -> DynamicImage {
    for pix in buf.pixels_mut() {
        let mut min_diff = f32::MAX;
        let mut best = &palette[0];
        for pal in palette {
            let dr = pix[0] as f32 - pal[0] as f32;
            let dg = pix[1] as f32 - pal[1] as f32;
            let db = pix[2] as f32 - pal[2] as f32;
            let diff = dr * dr + dg * dg + db * db;

            if diff < min_diff {
                best = &pal;
                min_diff = diff;

            }
        }
        *pix = Rgba([best[0], best[1], best[2], 0xFF]);
    }
    DynamicImage::ImageRgba8(buf)
}

pub fn get_palette(str: &str) -> Vec<[u8; 3]> {
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