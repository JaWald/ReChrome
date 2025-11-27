use std::time::SystemTime;
use crate::cli::{print_dashes, Args, Dither, TestType};
use crate::cli::Dither::*;
use crate::data::*;
use crate::processor::process_image;

const PALETTE_ARR: [&[[u8; 3]]; 6] = [EVERFOREST, GRUVBOX, KANAGAWA, MOLOKAI, PAPERCUT, SOLARIZED];
const DITHER_ARR: [Dither; 5] = [Raw, Bayer2, Bayer4, Bayer8, Bayer16];
const AMPL_ARR: [f32; 5]= [2.0, 16.0, 32.0, 64.0, 128.0];

pub fn test(args: &Args, test: &TestType) {
    let total_start = SystemTime::now();
    let format_str = "jpg";
    match test {
        TestType::None => return,
        TestType::Palette => {
            test_palette(args, format_str);
        },
        TestType::Dither => {
            test_dither(args, format_str);
        },
        TestType::Amplitude => {
            test_amplitude(args, format_str);
        },
        TestType::All => {
            test_palette(args, format_str);
            test_dither(args, format_str);
            test_amplitude(args, format_str);
        },
    }
    let total_end = SystemTime::now();
    println!("Total: [{:.2}s]", total_end.duration_since(total_start).unwrap().as_secs_f32());
    print_dashes(25);
}

fn test_palette(args: &Args, format_str: &str) {
    let buf = image::open(&args.input).unwrap().into_rgba8();
    for p in PALETTE_ARR.iter() {
        let start = SystemTime::now();
        let palette_str = match p {
            &EVERFOREST => "evrfrst",
            &GRUVBOX =>    "gruvbox",
            &KANAGAWA =>   "kanagwa",
            &MOLOKAI =>    "molokai",
            &PAPERCUT =>   "paprcut",
            &SOLARIZED =>  "solrizd",
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
        let processed = process_image(buf.clone(), p.to_vec(), Bayer8, 32.0);
        processed.save(path.clone()).expect("Couldn't save file");
        let end = SystemTime::now();
        println!(" \x1b[1m[{:.1?}]  Saved at:\x1b[0m   {}",end.duration_since(start).unwrap(), path.display());
    }
}

fn test_dither(args: &Args, format_str: &str) {
    let buf = image::open(&args.input).unwrap().into_rgba8();
    for d in DITHER_ARR.iter() {
        let start = SystemTime::now();
        let mut path = args.input.clone();
        let input_stem = path.file_stem().unwrap().to_string_lossy();
        let dither_str = match d {
            Raw => "".to_string(),
            Bayer2 => format!("_bayer02-{}", 32),
            Bayer4 => format!("_bayer04-{}", 32),
            Bayer8 => format!("_bayer08-{}", 32),
            Bayer16 => format!("_bayer16-{}", 32)
        };
        let file = format!(
            "{}_Dit_kanagwa{}.{}",
            input_stem,
            dither_str,
            format_str
        );
        path.set_file_name(file);
        let processed = process_image(buf.clone(), KANAGAWA.to_vec(), *d, 32.0);
        processed.save(path.clone()).expect("Couldn't save file");
        let end = SystemTime::now();
        println!(" \x1b[1m[{:.1?}]  Saved at:\x1b[0m   {}",end.duration_since(start).unwrap(), path.display());
    }
}

fn test_amplitude(args: &Args, format_str: &str) {
    let buf = image::open(&args.input).unwrap().into_rgba8();
    for a in AMPL_ARR.iter() {
        let start = SystemTime::now();
        let mut path = args.input.clone();
        let input_stem = path.file_stem().unwrap().to_string_lossy();
        let file = format!(
            "{}_Amp_kanagwa_bayer08-{}.{}",
            input_stem,
            a,
            format_str
        );
        path.set_file_name(file);
        let processed = process_image(buf.clone(), KANAGAWA.to_vec(), Bayer8, *a);
        processed.save(path.clone()).expect("Couldn't save file");
        let end = SystemTime::now();
        println!(" \x1b[1m[{:.1?}]  Saved at:\x1b[0m   {}",end.duration_since(start).unwrap(), path.display());
    }
}