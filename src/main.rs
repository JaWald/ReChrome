mod cli;
mod processor;

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
    
    // ---------------------------------------- PROCESSING ----------------------------------------
    
    let img = image::open(&args.input).expect("failed to open input image");
    let buf = img.into_rgba8();
    let pal_path = format!("palettes/{}.txt", args.palette.as_str());
    println!(" Converting to:  \x1b[33;1m{}\x1b[0m", args.palette.as_str());
    let processed = match args.palette.as_str() {
        "gray" => processor::process_gray(buf),
        "gruvbox" => process_image(buf, &*get_palette(pal_path.as_str())),
        //"everforest" => process_image(img, &EVERFOREST),
        _ => panic!("Unrecognized palette: {}", args.palette),
    };
    processed.save(&output).expect("failed to save image");
    println!(" Image saved at: {}", &output.to_str().unwrap());
    println!("-----------------------------------------------------------------");
    
    // --------------------------------------------------------------------------------------------
}