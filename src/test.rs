use std::time::{Duration, SystemTime};
use image::{ImageBuffer, Rgba};
use jpeg_encoder::{ColorType, Encoder};
use crate::cli::{Args, Dither, Format, TestType};
use crate::cli::Dither::*;
use crate::data::*;
use crate::error::AppError;
use crate::printer::{print_dashes, print_measurements};
use crate::processor::*;

const PALETTE_ARR: [&[[f32; 3]]; 11] = [ATOMONE, CATPPUCCIN, DARCULA, EVERFOREST, GRUVBOX, KANAGAWA, MONOKAI, NORD, PAPERCOLOR, SOLARIZED, SYNTHWAVE];
const DITHER_ARR: [Dither; 5] = [Raw, Bayer2, Bayer4, Bayer8, Bayer16];
const AMPL_ARR: [f32; 5]= [2.0, 16.0, 32.0, 64.0, 128.0];

pub fn test(args: &Args, test: &TestType, size: &u32) -> Result<(), AppError> {
    let total_start = SystemTime::now();
    let mut runtime :[Duration; 3] = [Duration::new(0, 0), Duration::new(0, 0), Duration::new(0, 0)];

    let img = image::open(&args.input)?;
    let buf = img.into_rgba8();
    match test {
        TestType::None => (),
        TestType::Palette => {
            runtime = test_palette(args, buf.clone(), runtime);
        },
        TestType::Dither => {
            runtime = test_dither(args, buf.clone(), runtime);
        },
        TestType::Amplitude => {
            runtime = test_amplitude(args, buf.clone(), runtime);
        },
        TestType::All => {
            runtime = test_palette(args, buf.clone(), runtime);
            println!();
            runtime = test_dither(args, buf.clone(), runtime);
            println!();
            runtime = test_amplitude(args, buf.clone(), runtime);
        },
    }
    let total_end = SystemTime::now();

    print_dashes(*size);
    runtime[0] = total_end.duration_since(total_start)? - runtime[1] - runtime[2];
    print_measurements(*size, runtime[0], runtime[1], runtime[2]);
    Ok(())
}

fn test_palette(args: &Args, buf: ImageBuffer<Rgba<u8>, Vec<u8>>, mut runtime: [Duration; 3]) -> [Duration; 3] {
    println!(" \x1b[32;1mTEST: Palette\x1b[0m");
    for pal in PALETTE_ARR.iter() {
        let proc_start = SystemTime::now();
        let palette_str = match pal {
            &ATOMONE    => "atomone",
            &CATPPUCCIN => "catppuc",
            &DARCULA    => "darcula_",
            &EVERFOREST => "everforst",
            &GRUVBOX    => "gruvbox",
            &KANAGAWA   => "kanagwa",
            &MONOKAI    => "monokai",
            &NORD       => "nord___",
            &PAPERCOLOR => "paprcol",
            &SOLARIZED  => "solarizd",
            &SYNTHWAVE  => "synthwv",
            _ => "",
        };
        let mut path = args.input.clone();
        let input_stem = path.file_stem().unwrap().to_string_lossy();
        let file = format!(
            "{}_Pal_{}.{:?}",
            input_stem,
            palette_str,
            args.format
        );
        path.set_file_name(file);
        let processed = process_image(buf.clone(), pal.to_vec(), Bayer16, 64.0);
        let proc_end = SystemTime::now();
        let encoder = Encoder::new_file(&path, 100).unwrap();
        match args.format {
            Format::Jpeg |
            Format::Jpg => encoder.encode(processed.to_rgba8().to_vec().as_slice(), buf.width() as u16, buf.height() as u16, ColorType::Rgba).expect("Should have been able to save image"),
            Format::Png =>  processed.save(&path).expect("Couldn't save file")
        }
        let save_end = SystemTime::now();

        let save_str = format!( "Saved at  {}", path.display());
        println!("   [{:>7.1?}]   {}", save_end.duration_since(proc_start).unwrap(), save_str);
        runtime[1] += proc_end.duration_since(proc_start).expect("Should have been able to use time");
        runtime[2] += save_end.duration_since(proc_end).expect("Should have been able to use time");
    }
    runtime
}

fn test_dither(args: &Args, buf: ImageBuffer<Rgba<u8>, Vec<u8>>, mut runtime: [Duration; 3]) -> [Duration; 3]  {
    println!(" \x1b[32;1mTEST: Dithering\x1b[0m");
    for dith in DITHER_ARR.iter() {
        let proc_start = SystemTime::now();
        let mut path = args.input.clone();
        let input_stem = path.file_stem().unwrap().to_string_lossy();
        let dither_str = match dith {
            Raw     => format!("_bayer-0-{}", 32),
            Bayer2  => format!("_bayer-2-{}", 32),
            Bayer4  => format!("_bayer-4-{}", 32),
            Bayer8  => format!("_bayer-8-{}", 32),
            Bayer16 => format!("_bayer16-{}", 32)
        };
        let file = format!(
            "{}_Dit_atomone{}.{:?}",
            input_stem,
            dither_str,
            args.format
        );
        path.set_file_name(file);
        let processed = process_image(buf.clone(), ATOMONE.to_vec(), *dith, 32.0);
        let proc_end = SystemTime::now();
        let encoder = Encoder::new_file(&path, 100).unwrap();
        match args.format {
            Format::Jpeg |
            Format::Jpg => encoder.encode(processed.to_rgba8().to_vec().as_slice(), buf.width() as u16, buf.height() as u16, ColorType::Rgba).expect("Should have been able to save image"),
            Format::Png =>  processed.save(&path).expect("Couldn't save file")
        }
        let save_end = SystemTime::now();

        let save_str = format!( "Saved at  {}", path.display());
        println!("   [{:>7.1?}]   {}", save_end.duration_since(proc_start).unwrap(), save_str);
        runtime[1] += proc_end.duration_since(proc_start).expect("Should have been able to use time");
        runtime[2] += save_end.duration_since(proc_end).expect("Should have been able to use time");
    }
    runtime
}

fn test_amplitude(args: &Args, buf: ImageBuffer<Rgba<u8>, Vec<u8>>, mut runtime: [Duration; 3]) -> [Duration; 3]  {
    println!(" \x1b[32;1mTEST: Bayer Amplitude\x1b[0m");
    for ampl in AMPL_ARR.iter() {
        let proc_start = SystemTime::now();
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
            "{}_Amp_atomone_bayer-8-{}.{:?}",
            input_stem,
            ampl_str,
            args.format
        );
        path.set_file_name(file);
        let processed = process_image(buf.clone(), ATOMONE.to_vec(), Bayer8, *ampl);
        let proc_end = SystemTime::now();
        let encoder = Encoder::new_file(&path, 100).unwrap();
        match args.format {
            Format::Jpeg |
            Format::Jpg => encoder.encode(processed.to_rgba8().to_vec().as_slice(), buf.width() as u16, buf.height() as u16, ColorType::Rgba).expect("Should have been able to save image"),
            Format::Png =>  processed.save(&path).expect("Couldn't save file")
        }
        let save_end = SystemTime::now();

        let save_str = format!( "Saved at  {}", path.display());
        println!("   [{:>7.1?}]   {}", save_end.duration_since(proc_start).unwrap(), save_str);
        runtime[1] += proc_end.duration_since(proc_start).expect("Should have been able to use time");
        runtime[2] += save_end.duration_since(proc_end).expect("Should have been able to use time");
    }
    runtime
}