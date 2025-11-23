mod cli;
mod processor;

use std::time::SystemTime;
use clap::Parser;
use cli::Args;
use processor::*;
use cli::*;

fn main() {
    // ------------------------------------------ INPUT -------------------------------------------
    let start_input = SystemTime::now();

    let args = Args::parse();
    validate_input(&args);
    let output = validate_output(&args);
    print_selection(&args, &output);

    let end_input = SystemTime::now();
    let dur_input = end_input.duration_since(start_input).expect("Input time should have been measured");

    // ------------------------------------------- LOAD -------------------------------------------
    let start_load = SystemTime::now();

    let img = image::open(&args.input).expect("failed to open input image");
    let buf = img.into_rgba8();
    let pal_path = format!("palettes/{}.txt", args.palette.as_str());

    let end_load = SystemTime::now();
    let dur_load = end_load.duration_since(start_load).expect("Load time should have been measured");

    // ------------------------------------------ PROCESS ------------------------------------------
    println!(" Converting to:  \x1b[33;1m{}\x1b[0m", args.palette.as_str());

    let start_proc = SystemTime::now();

    let processed = match args.palette.as_str() {
        "gray"          => process_gray(buf),
        "gruvbox"       => process_image(buf, &*get_palette(pal_path.as_str())),
        "everforest"    => process_image(buf, &*get_palette(pal_path.as_str())),
        "kanagawa"      => process_image(buf, &*get_palette(pal_path.as_str())),
        "solarized"     => process_image(buf, &*get_palette(pal_path.as_str())),
        "molokai"       => process_image(buf, &*get_palette(pal_path.as_str())),
        "papercut"      => process_image(buf, &*get_palette(pal_path.as_str())),
        _ => panic!("Unrecognized palette: {}", args.palette),
    };

    let end_proc = SystemTime::now();
    let dur_proc = end_proc.duration_since(start_proc).expect("Proc time should have been measured");

    // ------------------------------------------- SAVE -------------------------------------------
    let start_save = SystemTime::now();

    processed.save(&output).expect("failed to save image");

    let end_save = SystemTime::now();
    let dur_save = end_save.duration_since(start_save).expect("Save time should have been measured");

    print_measurements(dur_input, dur_load, dur_proc, dur_save);
    // --------------------------------------------------------------------------------------------
}