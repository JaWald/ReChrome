use std::cmp::max;
use std::path::PathBuf;
use std::time::Duration;
use clap::Parser;
use image::{DynamicImage, GenericImageView};

#[derive(Parser, Debug)]
#[command(version="0.0.1", about, long_about = None)]
pub struct Args {
    /// Color palette
    #[arg(short, long,  help = "Available:\x1b[33;1m \
    \n   > everforest    > gray\
    \n   > gruvbox       > kanagawa\
    \n   > molokai       > papercut\
    \n   > solarized \x1b[37;0m")]
    pub palette: String,

    /// Input  file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// Output file path (optional)
    #[arg(short, long, help = "Output file path (optional)\n")]
    pub output: Option<PathBuf>,

    /// Show preview (optional, recommended < 50)
    #[arg(short = 's', long = "showcase")]
    pub size: Option<u32>,

    /// Show runtime
    #[arg(short = 'r', long, help = "Show timing measurements\n")]
    pub runtime: bool,
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

pub fn print_dashes(show: &Option<u32>) {
    if show.is_some() {
        if show.unwrap() < 20 {
            println!("{}", "-".repeat(max(60usize, ((show.unwrap() + 1) * 4) as usize)));
        } else {
            println!("{}", "-".repeat(((show.unwrap() + 1) * 4) as usize));
        }
    } else {
        println!("{}", "-".repeat(60usize));
    }
}

pub fn print_selection(args: &Args, output: &PathBuf) {
    print_dashes(&args.size);
    println!(" \x1b[1mSelection: \x1b[0m");
    println!("   \x1b[32;1mPalette:\x1b[0m  \x1b[1m{}\x1b[0m", args.palette);
    println!("   \x1b[33;1mInput:\x1b[0m    {}", args.input.to_str().unwrap());
    println!("   \x1b[33;1mOutput:\x1b[0m   {}", output.to_str().unwrap());
    if args.size.is_some(){
        println!("   \x1b[33;1mPreview:\x1b[0m  {}x{} px", args.size.unwrap() * 2, args.size.unwrap());
    }
}

pub fn print_measurements(load: Duration, proc: Duration, save: Duration) {
    println!(" \x1b[1mPerformance:\x1b[0m");
    println!("   \x1b[32;1mTotal:\x1b[0m \x1b[1m  {:>8.1?}\x1b[0m", load + proc + save);
    println!("   \x1b[33;1mLoad:\x1b[0m    {:>8.1?}", load);
    println!("   \x1b[33;1mProcess:\x1b[0m {:>8.1?}", proc);
    println!("   \x1b[33;1mSave:\x1b[0m    {:>8.1?}", save);
}

pub fn print_preview(img: DynamicImage, max_rows: u32) {
    let width = img.width();
    let height = img.height();

    let max_cols = max_rows * 2;
    let step_x = (width / max_cols).max(1);
    let step_y = (height / max_rows).max(1);

    println!();
    for y in (0..height).step_by(step_y as usize) {
        print!("  ");
        for x in (0..width).step_by(step_x as usize) {
            let p = img.get_pixel(x, y).0;
            print!(" \x1b[48;2;{};{};{}m ", p[0], p[1], p[2]);
        }
        println!("\x1b[0m");
    }
    println!("\x1b[0m");
}