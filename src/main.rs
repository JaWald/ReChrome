mod cli;
mod processor;

use std::time::SystemTime;
use clap::Parser;
use cli::Args;
use processor::*;
use cli::*;

fn main() {
    // ------------------------------------------ INPUT -------------------------------------------
    let args = Args::parse();
    validate_input(&args);
    let output = validate_output(&args);
    print_selection(&args, &output);

    // ------------------------------------------- LOAD -------------------------------------------
    let start_load = SystemTime::now();
    let img = image::open(&args.input).expect("Should have loaded input image");
    let end_load = SystemTime::now();
    let dur_load = end_load.duration_since(start_load).unwrap();

    // ------------------------------------------ PROCESS ------------------------------------------
    let start_proc = SystemTime::now();

    let pal_path = format!("palettes/{}.txt", args.palette.as_str());
    let palette = get_palette(pal_path);
    let buf = img.into_rgba8();
    let processed = match args.palette.as_str() {
        "gray"          => process_gray(buf),
        "gruvbox"       => process_image(buf, &palette),
        "everforest"    => process_image(buf, &palette),
        "kanagawa"      => process_image(buf, &palette),
        "solarized"     => process_image(buf, &palette),
        "molokai"       => process_image(buf, &palette),
        "papercut"      => process_image(buf, &palette),
        _ => panic!("Unrecognized palette: {}", args.palette),
    };
    let end_proc = SystemTime::now();
    let dur_proc = end_proc.duration_since(start_proc).unwrap();

    // ------------------------------------------ OUTPUT ------------------------------------------
    let start_save = SystemTime::now();
    processed.save(&output).expect("failed to save image");
    let end_save = SystemTime::now();
    let dur_save = end_save.duration_since(start_save).unwrap();

    if args.runtime {
        print_dashes(&args.size);
        print_measurements(dur_load, dur_proc, dur_save);
    }

    if args.size.is_some() {
        print_dashes(&args.size);
        print_preview(processed, args.size.unwrap());
    }

    print_dashes(&args.size);
}