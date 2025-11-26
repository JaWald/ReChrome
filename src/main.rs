mod cli;
mod processor;
mod palettes;
mod error;

use std::process::exit;
use std::time::SystemTime;
use clap::Parser;
use cli::Args;
use processor::*;
use cli::*;
use cli::Palette::*;
use crate::cli::Dither::{Bayer16, Bayer2, Bayer4, Bayer8};
use crate::error::AppError;
use crate::palettes::{EVERFOREST, GRUVBOX, KANAGAWA, MOLOKAI, PAPERCUT, SOLARIZED};

fn main() {
    match run() {
        Ok(_) => {}
        Err(AppError::InputFileDoesNotExist(input_file)) => {
            eprintln!("\n\x1b[31;1mERROR[001] --->\x1b[0m\x1b[1m Invalid input path, no file found at\x1b[0m ");
            eprintln!("{:>16}{}\n", "", input_file.to_str().unwrap());
            exit(1)
        },
        Err(AppError::ImageError(e)) => {
            eprintln!("\n\x1b[31;1mERROR[002] --->\x1b[0m\x1b[1m Wrong file format\x1b[0m");
            eprintln!("{:>16}{}\n", "", e);
            exit(1)
        },
        Err(AppError::SystemTimeError(e)) => {
            eprintln!("\n\x1b[31;1mERROR[003] --->\x1b[0m\x1b[1m Timing Measurement failed\x1b[0m");
            eprintln!("{:>16}{}\n", "", e);
            exit(1)
        },
        Err(AppError::IoError(e)) => {
            eprintln!("\n\x1b[31;1mERROR[004] --->\x1b[0m\x1b[1m IO Error occured\x1b[0m");
            eprintln!("{:>16}{}\n", "", e);
            exit(1)
        }
    }
}

fn run() -> Result<(), AppError> {
    // ------------------------------------------ INPUT -------------------------------------------
    let args = Args::parse();
    validate_input(&args)?;
    let output = create_output_path(&args);
    print_selection(&args, &output);

    // ------------------------------------------- LOAD -------------------------------------------
    let start_load = SystemTime::now();
    let img = image::open(&args.input)?;
    let end_load = SystemTime::now();
    let dur_load = end_load.duration_since(start_load)?;

    // ------------------------------------------ PROCESS ------------------------------------------
    let start_proc = SystemTime::now();

    let buf = img.into_rgba8();
    let palette = get_palette(&args.palette);
    let dither = args.dither;
    let bayer = args.bayer;
    let ideal_ampl = match dither {
        Dither::None => 0,
        Bayer2 => 8,
        Bayer4 => 16,
        Bayer8 => 32,
        Bayer16 => 64
    };
    let amplitude :f32 = bayer.unwrap_or_else(|| ideal_ampl) as f32;

    let processed = match args.palette {
        Gray        => process_gray(buf),
        Gruvbox     => process_image(buf, Vec::from(palette), dither, amplitude),
        Everforest  => process_image(buf, Vec::from(palette), dither, amplitude),
        Kanagawa    => process_image(buf, Vec::from(palette), dither, amplitude),
        Solarized   => process_image(buf, Vec::from(palette), dither, amplitude),
        Molokai     => process_image(buf, Vec::from(palette), dither, amplitude),
        Papercut    => process_image(buf, Vec::from(palette), dither, amplitude),
    };
    let end_proc = SystemTime::now();
    let dur_proc = end_proc.duration_since(start_proc)?;

    // ------------------------------------------ OUTPUT ------------------------------------------
    let start_save = SystemTime::now();
    processed.save(&output)?;
    let end_save = SystemTime::now();
    let dur_save = end_save.duration_since(start_save)?;

    if args.runtime {
        print_measurements(dur_load, dur_proc, dur_save);
        print_dashes(&args.size);
    }

    if args.size.is_some() {
        print_preview(processed, args.size.unwrap());
        print_dashes(&args.size);
    }

    println!(" \x1b[1mImage saved at:\x1b[0m\n   {}", output.display());
    print_dashes(&args.size);

    if args.test {
        let buf = image::open(&args.input)?.into_rgba8();
        let palette = [EVERFOREST, GRUVBOX, KANAGAWA, MOLOKAI, PAPERCUT, SOLARIZED];
        let dither_arr = [Bayer2, Bayer4, Bayer8, Bayer16, Dither::None];
        let bayer = args.bayer;
        let input = args.input.clone();
        let dither = args.dither;
        test(bayer, input, buf, palette, dither_arr, dither);
    }

    Ok(())
}