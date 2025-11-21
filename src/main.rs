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

    // ---------------------------------------- PROCESSING ----------------------------------------

    let start_load = SystemTime::now();

    let img = image::open(&args.input).expect("failed to open input image");
    let buf = img.into_rgba8();
    let pal_path = format!("palettes/{}.txt", args.palette.as_str());

    let end_load = SystemTime::now();
    let dur_load = end_load.duration_since(start_load).expect("Load time should have been measured");

    println!(" Converting to:  \x1b[33;1m{}\x1b[0m", args.palette.as_str());

    let start_proc = SystemTime::now();

    let processed = match args.palette.as_str() {
        "gray" => processor::process_gray(buf),
        "gruvbox" => process_image(buf, &*get_palette(pal_path.as_str())),
        "everforest" => process_image(buf, &*get_palette(pal_path.as_str())),
        _ => panic!("Unrecognized palette: {}", args.palette),
    };

    let end_proc = SystemTime::now();
    let dur_proc = end_proc.duration_since(start_proc).expect("Proc time should have been measured");
    let start_save = SystemTime::now();

    processed.save(&output).expect("failed to save image");

    let end_save = SystemTime::now();
    let dur_save = end_save.duration_since(start_save).expect("Save time should have been measured");

    println!(" Image saved at: {}", &output.to_str().unwrap());
    println!("-----------------------------------------------------------------");
    println!(" \x1b[33;1mInput:\x1b[0m    {:?}", dur_input);
    println!(" \x1b[33;1mLoad:\x1b[0m     {:?}", dur_load);
    println!(" \x1b[33;1mProcess:\x1b[0m  {:?}", dur_proc);
    println!(" \x1b[33;1mSave:\x1b[0m     {:?}", dur_save);
    println!("--------------------------");
    println!("   \x1b[32;1mTotal:\x1b[0m    {:?}", dur_input + dur_load + dur_proc + dur_save);
    println!("-----------------------------------------------------------------");
    
    // --------------------------------------------------------------------------------------------
}