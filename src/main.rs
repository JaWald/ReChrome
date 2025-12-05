mod cli;
mod processor;
mod data;
mod error;
mod config;
mod test;
mod printer;

use std::process::exit;
use std::time::SystemTime;
use clap::Parser;
use processor::*;
use cli::*;
use crate::config::*;
use crate::error::AppError;
use crate::printer::*;
use crate::test::test;

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
        Err(AppError::CompressionError(e)) => {
            eprintln!("\n\x1b[31;1mERROR[005] --->\x1b[0m\x1b[1m Jpeg compression has failed\x1b[0m");
            eprintln!("{:>16}{}\n", "", e);
            exit(1)
        }
    }
}

fn run() -> Result<(), AppError> {
    // ------------------------------------------ INPUT -------------------------------------------
    let args = Args::parse();
    let config = from_args(&args)?;
    print_dashes(config.size);

    // ------------------------------------------- TEST -------------------------------------------
    if config.test != TestType::None {
        test(&args, &config)?;
        return Ok(());
    }

    // ------------------------------------------- LOAD -------------------------------------------
    let start_load = SystemTime::now();
    print_selection(&config, config.output.as_str());
    let img = image::open(&config.input)?;
    let end_load = SystemTime::now();
    let dur_load = end_load.duration_since(start_load)?;

    // ------------------------------------------ PROCESS -----------------------------------------
    let start_proc = SystemTime::now();
    //let processed = process_floyd(img.into_rgba8(), config.palette.colors.to_vec(), config.dither, config.ampl);
    let processed = process_image(img.into_rgba8(), config.palette.colors.to_vec(), config.dither, config.ampl);
    let end_proc = SystemTime::now();
    let dur_proc = end_proc.duration_since(start_proc)?;

    // ------------------------------------------ OUTPUT ------------------------------------------
    let start_save = SystemTime::now();
    save_image(&processed, &config.output, &config.format, config.quality)?;
    let end_save = SystemTime::now();
    let dur_save = end_save.duration_since(start_save)?;

    if config.runtime  { print_measurements(config.size, dur_load, dur_proc, dur_save); }
    if config.size > 0 { print_preview(processed, config.size); }

    println!(" \x1b[1mImage saved at:\x1b[0m\n   {}", &config.output);
    print_dashes(config.size);
    // --------------------------------------------------------------------------------------------

    Ok(())
}