use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version="0.0.1", about, long_about = None)]
pub struct Args {
    /// Input file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// Color palette
    #[arg(short, long, help="Available: \n   \x1b[33;1m> gray\n   > gruvbox \x1b[37;0m")]
    pub palette: String,

    /// Output file path (optional)
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

pub fn print_selection(args: &Args, output: &PathBuf) {
    println!("\n--------------------------------------------------------------------");
    println!(" \x1b[1mYour selection: \x1b[0m");
    println!("   \x1b[32;1mPalette:\x1b[0m  \x1b[1m{}\x1b[0m", args.palette);
    println!("   \x1b[32;1mInput:\x1b[0m    {}", args.input.to_str().unwrap());
    println!("   \x1b[32;1mOutput:\x1b[0m   {}", output.to_str().unwrap());
    println!("--------------------------------------------------------------------");
}