use image::DynamicImage;

pub fn process_gruvbox(img: &image::DynamicImage) -> DynamicImage {
    println!("processing gruvbox\n");
    img.grayscale()
}

pub fn process_gameboy(img: &image::DynamicImage) -> DynamicImage {
    println!("processing gameboy\n");
    img.grayscale()
}