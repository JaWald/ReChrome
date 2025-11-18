mod cli;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();
    let img = image::open(args.input).expect("failed to open input image");
    let img = img.grayscale();

    img.save(args.output).expect("failed to save image");
}