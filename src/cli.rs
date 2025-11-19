use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version="0.0.1", about, long_about = None)]
pub struct Args {
    /// Input file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// Color palette
    #[arg(short, long, help="Available: \n   > gray\n   > gray8")]
    pub palette: String,

    /// Output file path (optional)
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

pub fn print_selection(args: &Args){
    println!("---------------------------------------------------------");
    println!(" \x1b[1mYour selection: \x1b[0m");
    println!("   \x1b[32;1mPalette:\x1b[0m  {}", args.palette);
    println!("   \x1b[32;1mInput:\x1b[0m    {}", args.input.to_str().unwrap());
    if &args.output == &None {
        let mut path = args.input.clone();
        path.set_file_name(format!("{}_ReChrome.png", path.file_stem().unwrap().to_string_lossy()));
        println!("   \x1b[32;1mOutput:\x1b[0m   {} (auto)", path.display());
    } else {
        println!("   \x1b[32;1mOutput:\x1b[0m   {}", args.output.clone().unwrap().to_str().unwrap());
    }
    println!("---------------------------------------------------------");
}