use std::time::SystemTime;
use image::{ImageBuffer, Rgba};
use crate::cli::{Args, Dither, TestType};
use crate::cli::Dither::*;
use crate::data::*;
use crate::error::AppError;
use crate::printer::print_dashes;
use crate::processor::process_image;

const PALETTE_ARR: [&[[u8; 3]]; 11] = [ATOMONE, CATPPUCCIN, DARCULA, EVERFOREST, GRUVBOX, KANAGAWA, MONOKAI, NORD, PAPERCUT, SOLARIZED, SYNTHWAVE];
const DITHER_ARR: [Dither; 5] = [Raw, Bayer2, Bayer4, Bayer8, Bayer16];
const AMPL_ARR: [f32; 5]= [2.0, 16.0, 32.0, 64.0, 128.0];

pub fn test(args: &Args, test: &TestType, size: &u32) -> Result<(), AppError> {
    let total_start = SystemTime::now();
    let format_str = "jpg";
    let img = image::open(&args.input)?;
    let buf = img.into_rgba8();
    match test {
        TestType::None => (),
        TestType::Palette => {
            test_palette(args, format_str, buf.clone());
        },
        TestType::Dither => {
            test_dither(args, format_str, buf.clone());
        },
        TestType::Amplitude => {
            test_amplitude(args, format_str, buf.clone());
        },
        TestType::All => {

            test_palette(args, format_str, buf.clone());
            println!();
            test_dither(args, format_str, buf.clone());
            println!();
            test_amplitude(args, format_str, buf.clone());
        },
    }
    let total_end = SystemTime::now();
    println!("     \x1b[1m[{:.2}s]   {}", total_end.duration_since(total_start)?.as_secs_f32(), "Total");
    print_dashes(*size);
    Ok(())
}

fn test_palette(args: &Args, format_str: &str, buf: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    println!(" \x1b[32;1mTEST: Palette\x1b[0m");
    for pal in PALETTE_ARR.iter() {
        let start = SystemTime::now();
        let palette_str = match pal {
            &ATOMONE    => "atomone",
            &CATPPUCCIN => "catppuc",
            &DARCULA    => "darcula",
            &EVERFOREST => "everforst",
            &GRUVBOX    => "gruvbox",
            &KANAGAWA   => "kanagwa",
            &MONOKAI    => "monokai",
            &NORD       => "nord___",
            &PAPERCUT   => "paprcut",
            &SOLARIZED  => "solarizd",
            &SYNTHWAVE  => "synthwve",
            _ => "",
        };
        let mut path = args.input.clone();
        let input_stem = path.file_stem().unwrap().to_string_lossy();
        let file = format!(
            "{}_Pal_{}.{}",
            input_stem,
            palette_str,
            format_str
        );
        path.set_file_name(file);
        let processed = process_image(buf.clone(), pal.to_vec(), Bayer8, 32.0);
        processed.save(path.clone()).expect("Couldn't save file");
        let end = SystemTime::now();
        println!("    [{:.1?}]{:>12}   {}", end.duration_since(start).unwrap(), "Saved at:", path.display());
    }
}

fn test_dither(args: &Args, format_str: &str, buf: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    println!(" \x1b[32;1mTEST: Dithering\x1b[0m");
    for dith in DITHER_ARR.iter() {
        let start = SystemTime::now();
        let mut path = args.input.clone();
        let input_stem = path.file_stem().unwrap().to_string_lossy();
        let dither_str = match dith {
            Raw     => format!("_bayer-0-{}", 64),
            Bayer2  => format!("_bayer-2-{}", 64),
            Bayer4  => format!("_bayer-4-{}", 64),
            Bayer8  => format!("_bayer-8-{}", 64),
            Bayer16 => format!("_bayer16-{}", 64)
        };
        let file = format!(
            "{}_Dit_atomone{}.{}",
            input_stem,
            dither_str,
            format_str
        );
        path.set_file_name(file);
        let processed = process_image(buf.clone(), ATOMONE.to_vec(), *dith, 32.0);
        processed.save(path.clone()).expect("Couldn't save file");
        let end = SystemTime::now();
        println!("    [{:.1?}]{:>12}   {}", end.duration_since(start).unwrap(), "Saved at:", path.display());
    }
}

fn test_amplitude(args: &Args, format_str: &str, buf: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    println!(" \x1b[32;1mTEST: Bayer Amplitude\x1b[0m");
    for ampl in AMPL_ARR.iter() {
        let start = SystemTime::now();
        let mut path = args.input.clone();
        let input_stem = path.file_stem().unwrap().to_string_lossy();
        let ampl_str = match ampl {
            2.0 => "--2",
            16.0 => "-16",
            32.0 => "-32",
            64.0 => "-64",
            128.0 => "128",
            _ => ""
        };
        let file = format!(
            "{}_Amp_atomone_bayer-8-{}.{}",
            input_stem,
            ampl_str,
            format_str
        );
        path.set_file_name(file);
        let processed = process_image(buf.clone(), ATOMONE.to_vec(), Bayer8, *ampl);
        processed.save(path.clone()).expect("Couldn't save file");
        let end = SystemTime::now();
        println!("    [{:.1?}]{:>12}   {}", end.duration_since(start).unwrap(), "Saved at:", path.display());
    }
}