use crate::cli::{print_dashes, Args, Dither, TestType};
use crate::cli::Dither::*;
use crate::palettes::*;
use crate::processor::process_image;

pub fn test(args: &Args, test: &TestType) {
    let palette_arr = [EVERFOREST, GRUVBOX, KANAGAWA, MOLOKAI, PAPERCUT, SOLARIZED];
    let dither_arr = [Raw, Bayer2, Bayer4, Bayer8, Bayer16];
    let ampl_arr = [2.0, 16.0, 32.0, 64.0, 128.0];
    let format_str = "jpg";
    match test {
        TestType::None => return,
        TestType::Palette => {
            test_palette(args, palette_arr, format_str);
        },
        TestType::Dither => {
            test_dither(args, dither_arr, format_str);
        },
        TestType::Amplitude => {
            test_amplitude(args, ampl_arr, format_str);
        },
        TestType::All => {
            test_palette(args, palette_arr, format_str);
            test_dither(args, dither_arr, format_str);
            test_amplitude(args, ampl_arr, format_str);
        },
    }
    print_dashes(20);
}

fn test_palette(args: &Args, palette_arr: [&[[u8; 3]]; 6], format_str: &str) {
    for p in palette_arr.iter() {
        let palette_str = match p {
            &EVERFOREST => "everforest",
            &GRUVBOX => "gruvbox",
            &KANAGAWA => "kanagawa",
            &MOLOKAI => "molokai",
            &PAPERCUT => "papercut",
            &SOLARIZED => "solarized",
            _ => "",
        };
        let mut path = args.input.clone();
        let input_stem = path.file_stem().unwrap().to_string_lossy();
        let file = format!(
            "{}_testPalette_{}.{}",
            input_stem,
            palette_str,
            format_str
        );
        path.set_file_name(file);
        let processed = process_image(image::open(&args.input).unwrap().into_rgba8(), p.to_vec(), Bayer8, 32.0);
        processed.save(path.clone()).expect("Couldn't save file");
        println!(" \x1b[1mImage saved at:\x1b[0m   {}", path.display());
    }
}

fn test_dither(args: &Args, dither_arr: [Dither; 5], format_str: &str) {
    for d in dither_arr.iter() {
        let mut path = args.input.clone();
        let input_stem = path.file_stem().unwrap().to_string_lossy();
        let dither_str = match d {
            Raw => "".to_string(),
            Bayer2 => format!("_bayer2-{}", 32),
            Bayer4 => format!("_bayer4-{}", 32),
            Bayer8 => format!("_bayer8-{}", 32),
            Bayer16 => format!("_bayer16-{}", 32)
        };
        let file = format!(
            "{}_testDither_kanagawa{}.{}",
            input_stem,
            dither_str,
            format_str
        );
        path.set_file_name(file);
        let processed = process_image(image::open(&args.input).unwrap().into_rgba8(), KANAGAWA.to_vec(), *d, 32.0);
        processed.save(path.clone()).expect("Couldn't save file");
        println!(" \x1b[1mImage saved at:\x1b[0m   {}", path.display());
    }
}

fn test_amplitude(args: &Args, ampl_arr: [f32; 5], format_str: &str) {
    for a in ampl_arr.iter() {
        let mut path = args.input.clone();
        let input_stem = path.file_stem().unwrap().to_string_lossy();
        let file = format!(
            "{}_testAmpl_kanagawa_bayer8_{}.{}",
            input_stem,
            a,
            format_str
        );
        path.set_file_name(file);
        let processed = process_image(image::open(&args.input).unwrap().into_rgba8(), KANAGAWA.to_vec(), Bayer8, *a);
        processed.save(path.clone()).expect("Couldn't save file");
        println!(" \x1b[1mImage saved at:\x1b[0m   {}", path.display());
    }
}
