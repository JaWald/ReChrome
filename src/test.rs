use std::time::{Duration, SystemTime};
use image::{ImageBuffer, Rgba};
use crate::cli::{Args, Dither, TestType};
use crate::cli::Dither::*;
use crate::config::{create_output_path, Config};
use crate::data::*;
use crate::error::AppError;
use crate::printer::{print_dashes, print_measurements};
use crate::processor::*;

const DITHER_ARR: [Dither; 6] = [Raw, Bayer2, Bayer4, Bayer8, Bayer16, Fs];
const AMPL_ARR: [f32; 5]= [2.0, 16.0, 32.0, 64.0, 128.0];

pub fn test(args: &Args, config: &Config) -> Result<(), AppError> {
    let total_start = SystemTime::now();
    let mut runtime :[Duration; 3] = [Duration::new(0, 0), Duration::new(0, 0), Duration::new(0, 0)];

    let img = image::open(&args.input)?;
    let buf = img.into_rgba8();
    match config.test {
        TestType::None => (),
        TestType::Palette => {
            runtime = test_palette(args, buf.clone(), runtime, &config)?;
        },
        TestType::Dither => {
            runtime = test_dither(args, buf.clone(), runtime)?;
        },
        TestType::Amplitude => {
            runtime = test_amplitude(args, buf.clone(), runtime)?;
        },
        TestType::All => {
            runtime = test_palette(args, buf.clone(), runtime, &config)?;
            println!();
            runtime = test_dither(args, buf.clone(), runtime)?;
            println!();
            runtime = test_amplitude(args, buf.clone(), runtime)?;
        },
    }
    let total_end = SystemTime::now();

    print_dashes(config.size);
    runtime[0] = total_end.duration_since(total_start)? - runtime[1] - runtime[2];
    print_measurements(config.size, runtime[0], runtime[1], runtime[2]);
    Ok(())
}

fn test_palette(args: &Args, buf: ImageBuffer<Rgba<u8>, Vec<u8>>, mut runtime: [Duration; 3], config: &Config) -> Result<[Duration; 3], AppError> {
    println!(" \x1b[32;1mTEST: Palette\x1b[0m");
    let ampl = config.ampl;
    let dither = config.dither;
    for pal in PALETTES.iter() {
        let proc_start = SystemTime::now();
        let path = create_output_path(&args, "_P", pal, ampl, dither, &config.format)?;
        let processed = process_image(buf.clone(), pal.colors.to_vec(), dither, ampl);
        let proc_end = SystemTime::now();

        save_image(&processed, &path, &config.format, config.quality)?;
        let save_end = SystemTime::now();

        println!("   [{:>7.1?}]   {}", save_end.duration_since(proc_start)?, format!( "Saved at  {}", path));
        runtime[1] += proc_end.duration_since(proc_start).expect("Should have been able to use time");
        runtime[2] += save_end.duration_since(proc_end).expect("Should have been able to use time");
    }
    Ok(runtime)
}

fn test_dither(args: &Args, buf: ImageBuffer<Rgba<u8>, Vec<u8>>, mut runtime: [Duration; 3]) -> Result<[Duration; 3], AppError> {
    println!(" \x1b[32;1mTEST: Dithering\x1b[0m");
    let ampl = 32.0;
    for dith in DITHER_ARR.iter() {
        let proc_start = SystemTime::now();
        let path = create_output_path(&args, "_D", &ATOMONE, ampl, *dith, &args.format)?;
        let processed = process_image(buf.clone(), ATOMONE.colors.to_vec(), *dith, 32.0);
        let proc_end = SystemTime::now();
        save_image(&processed, &path, &args.format, args.quality.unwrap_or_else(|| 100u8))?;
        let save_end = SystemTime::now();

        println!("   [{:>7.1?}]   {}", save_end.duration_since(proc_start)?, format!( "Saved at  {}", path));
        runtime[1] += proc_end.duration_since(proc_start).expect("Should have been able to use time");
        runtime[2] += save_end.duration_since(proc_end).expect("Should have been able to use time");
    }
    Ok(runtime)
}

fn test_amplitude(args: &Args, buf: ImageBuffer<Rgba<u8>, Vec<u8>>, mut runtime: [Duration; 3]) -> Result<[Duration; 3], AppError> {
    println!(" \x1b[32;1mTEST: Bayer Amplitude\x1b[0m");
    let dither = Bayer8;
    for ampl in AMPL_ARR.iter() {
        let proc_start = SystemTime::now();
        let path = create_output_path(&args, "_A", &ATOMONE, *ampl, dither, &args.format)?;
        let processed = process_image(buf.clone(), ATOMONE.colors.to_vec(), Bayer8, *ampl);
        let proc_end = SystemTime::now();
        save_image(&processed, &path, &args.format, args.quality.unwrap_or_else(|| 100u8))?;
        let save_end = SystemTime::now();

        println!("   [{:>7.1?}]   {}", save_end.duration_since(proc_start)?, format!( "Saved at  {}", path));
        runtime[1] += proc_end.duration_since(proc_start).expect("Should have been able to use time");
        runtime[2] += save_end.duration_since(proc_end).expect("Should have been able to use time");
    }
    Ok(runtime)
}