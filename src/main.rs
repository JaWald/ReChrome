mod cli;
mod processor;
mod palettes;

use clap::Parser;
use cli::Args;
use cli::print_selection;
use crate::cli::{validate_input, validate_output};
use crate::palettes::{EVERFOREST, GRUVBOX};
use crate::processor::process_image;

fn main() {
    // ------------------------------------------ INPUT -------------------------------------------
    
    let args = Args::parse();
    validate_input(&args);
    let output = validate_output(&args);
    print_selection(&args, &output);
    
    // ---------------------------------------- PROCESSING ----------------------------------------
    
    let img = image::open(&args.input).expect("failed to open input image");
    println!(" Converting to:  \x1b[33;1m{}\x1b[0m", args.palette.as_str());
    let processed = match args.palette.as_str() {
        "gray" => processor::process_gray(img),
        "gruvbox" => process_image(img, &GRUVBOX),
        "everforest" => process_image(img, &EVERFOREST),
        _ => panic!("Unrecognized palette: {}", args.palette),
    };
    processed.save(&output).expect("failed to save image");
    println!(" Image saved at: {}", &output.to_str().unwrap());
    println!("-----------------------------------------------------------------");
    
    // --------------------------------------------------------------------------------------------
}