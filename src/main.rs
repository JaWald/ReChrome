mod cli;
mod processor;

use std::time::SystemTime;
use clap::Parser;
use cli::Args;
use processor::*;
use cli::*;
use cli::Palette::*;

fn main() {
    // ------------------------------------------ INPUT -------------------------------------------
    let args = Args::parse();
    validate_input(&args);
    let output = create_output(&args);
    print_selection(&args, &output);

    // ------------------------------------------- LOAD -------------------------------------------
    let start_load = SystemTime::now();
    let img = image::open(&args.input).expect("Should have loaded input image");
    let end_load = SystemTime::now();
    let dur_load = end_load.duration_since(start_load).unwrap();

    // ------------------------------------------ PROCESS ------------------------------------------
    let start_proc = SystemTime::now();

    let pal_path = format!("palettes/{:?}.txt", args.palette);

    let buf = img.into_rgba8();
    let palette = get_palette(pal_path);
    let dither = args.dither;
    let bayer = args.bayer;

    let processed = match args.palette {
        Gray        => process_gray(buf),
        Gruvbox     => process_image(buf, palette, dither, bayer),
        Everforest  => process_image(buf, palette, dither, bayer),
        Kanagawa    => process_image(buf, palette, dither, bayer),
        Solarized   => process_image(buf, palette, dither, bayer),
        Molokai     => process_image(buf, palette, dither, bayer),
        Papercut    => process_image(buf, palette, dither, bayer),
    };
    let end_proc = SystemTime::now();
    let dur_proc = end_proc.duration_since(start_proc).unwrap();

    // ------------------------------------------ OUTPUT ------------------------------------------
    let start_save = SystemTime::now();
    processed.save(&output).expect("failed to save image");
    let end_save = SystemTime::now();
    let dur_save = end_save.duration_since(start_save).unwrap();

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
}