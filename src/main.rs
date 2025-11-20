mod cli;
mod processor;
mod palettes;

use clap::Parser;
use cli::Args;
use cli::print_selection;

fn main() {
    // ------------------------------------------ INPUT -------------------------------------------
    let args = Args::parse();

    // checks if image exists
    if !args.input.exists() {
        eprintln!("\n\x1b[31;1mError -->\x1b[0m Input file does not exist at:\n          {:?} \n", args.input);
        std::process::exit(1);
    }
    // checks existence of output path, clones input path with new file name if necessary
    let output = match &args.output {
        Some(path) => path,
        None => &{
            let mut path = args.input.clone();
            path.set_file_name(format!("{}_{}.png", path.file_stem().unwrap().to_string_lossy(), args.palette));
            path
        }
    };
    print_selection(&args, output);

    // ---------------------------------------- PROCESSING ----------------------------------------
    
    let mut img = image::open(&args.input).expect("failed to open input image");
    println!(" Converting to:  \x1b[33;1m{}\x1b[0m", args.palette.as_str());
    let processed = match args.palette.as_str() {
        "gray" => processor::process_gray(&mut img),
        "gruvbox" => processor::process_gruvbox(&mut img),
        _ => panic!("Unknown palette: {}", args.palette),
    };
    processed.save(output).expect("failed to save image");
    println!(" Image saved at: {:?}", output);
    println!("--------------------------------------------------------------------");
    
    // --------------------------------------------------------------------------------------------
}