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
    println!("\n-----------------------------------------------------------------");
    println!(" \x1b[1mYour selection: \x1b[0m");
    println!("   \x1b[32;1mPalette:\x1b[0m  \x1b[1m{}\x1b[0m", args.palette);
    println!("   \x1b[32;1mInput:\x1b[0m    {}", args.input.to_str().unwrap());
    println!("   \x1b[32;1mOutput:\x1b[0m   {}", output.to_str().unwrap());
    println!("-----------------------------------------------------------------");
}

// checks whether image even exists
pub fn validate_input(args: &Args) {
    if !args.input.exists() {
        eprintln!("\n\x1b[31;1mError -->\x1b[0m Input file does not exist at:\n          {:?} \n", args.input);
        std::process::exit(1);
    }
}

// checks existence of output path, clones input path with new file name if necessary
pub fn validate_output(args: &Args) -> PathBuf{
    match &args.output {
        Some(path) => path.to_path_buf(),
        None => {
            let mut path = args.input.clone();
            path.set_file_name(format!("{}_{}.png", path.file_stem().unwrap().to_string_lossy(), args.palette));
            path
        }
    }
}