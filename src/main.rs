mod cli;
mod palette;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    // checks if image exists
    if !args.input.exists() {
        eprintln!("\n\x1b[31mError -->\x1b[0m Input file does not exist at:\n          {:?} \n", args.input);
        std::process::exit(1);
    }

    // checks existence of output path, clones input path with new file name if necessary
    let output = match args.output {
        Some(path) => path,
        None => {
            let mut path = args.input.clone();
            path.set_file_name(format!("{}_rechrome.png", path.file_stem().unwrap().to_string_lossy()));
            path
        }
    };

    let img = image::open(args.input).expect("failed to open input image");

    let processed = match args.palette.as_str() {
        "gruvbox" => palette::process_gruvbox(&img),
        "gamebody" => palette::process_gameboy(&img),
        _ => panic!("Unknown palette: {}", args.palette),
    };

    processed.save(output).expect("failed to save image");
}