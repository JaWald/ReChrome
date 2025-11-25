use std::cmp::max;
use std::fmt::Debug;
use std::path::PathBuf;
use std::time::Duration;
use clap::{Parser, ValueEnum};
use image::{DynamicImage, GenericImageView};

#[derive(Parser, Debug)]
#[command(version="0.0.1", about, long_about = None)]
pub struct Args {
    /// Color palette
    #[arg(short, long, value_enum, help = "\x1b[33;1mColor Palette  \x1b[0m")]
    pub palette: Palette,

    /// Dithering Modes
    #[arg(short, long, value_enum, default_value = "bayer16")]
    pub dither: Dither,

    /// Bayer intensity
    #[arg(short, long, help = "Bayer amplitude [default: bayerX * 4] \n")]
    pub bayer: Option<u8>,

    /// Input  file path
    #[arg(short, long, help = "\x1b[33;1mInput file path  \x1b[0m")]
    pub input: PathBuf,

    /// Output file type
    #[arg(short, long, value_enum, default_value = "png")]
    pub format: Format,

    /// Output file path
    #[arg(short, long, help = "Output file path [default: <input>_<palette>_<dither>.<format>]\n")]
    pub output: Option<PathBuf>,

    /// Show in-terminal (recommended value: <40)
    #[arg(short = 's', long = "showcase")]
    pub size: Option<u32>,

    /// Show runtime
    #[arg(short = 'r', long, help = "Show timing measurements\n")]
    pub runtime: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Palette {
    Gray,
    Gruvbox,
    Everforest,
    Kanagawa,
    Molokai,
    Papercut,
    Solarized
}

#[derive(Debug, Clone, ValueEnum, Copy)]
pub enum Dither {
    None,
    Bayer2,
    Bayer4,
    Bayer8,
    Bayer16
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    Png,
    Jpg,
    Jpeg
}

// checks whether image even exists
pub fn validate_input(args: &Args) {
    if !args.input.exists() {
        eprintln!("\n\x1b[31;1mError -->\x1b[0m Input file does not exist at:\n          {:?} \n", args.input);
        std::process::exit(1);
    }
}

// checks existence of output path, clones input path with new file name if necessary
pub fn create_output(args: &Args) -> PathBuf{
  match &args.output {
        Some(path) => path.to_path_buf(),
        None => {
            let mut path = args.input.clone();
            let input_stem = path.file_stem().unwrap().to_string_lossy();
            let palette_str = match args.palette {
                Palette::Everforest => "gruvbox",
                Palette::Gray => "gray",
                Palette::Gruvbox => "gruvbox",
                Palette::Kanagawa => "kanagawa",
                Palette::Molokai => "molokai",
                Palette::Papercut => "papercut",
                Palette::Solarized => "solarized"
            };
            let bayer = match &args.bayer {
                Some(bayer) => bayer,
                None => &match args.dither {
                    Dither::None => 0,
                    Dither::Bayer2 => 8,
                    Dither::Bayer4 => 16,
                    Dither::Bayer8 => 32,
                    Dither::Bayer16 => 64
                }
            };
            let dither_str = match args.dither {
                Dither::None => "".to_string(),
                Dither::Bayer2 => format!("_bayer2-{}", bayer),
                Dither::Bayer4 => format!("_bayer4-{}", bayer),
                Dither::Bayer8 => format!("_bayer8-{}", bayer),
                Dither::Bayer16 => format!("_bayer16-{}", bayer)
            };
            let format_str = match args.format {
                Format::Png => "png",
                Format::Jpg => "jpg",
                Format::Jpeg => "jpeg"
            };

            let file = format!(
                "{}_{}{}.{}",
                input_stem,
                palette_str,
                dither_str,
                format_str
            );

            path.set_file_name(file);
            path
        }
    }

}

// prints dashes in corresponding size to preview image, minimum is 80
pub fn print_dashes(show: &Option<u32>) {
    println!("{}", "-".repeat(max(80usize, ((show.unwrap_or(0) + 1) * 4) as usize)));
}

pub fn print_selection(args: &Args, output: &PathBuf) {
    print_dashes(&args.size);
    println!(" \x1b[1mSelection: \x1b[0m");
    println!("   \x1b[32;1mPalette:\x1b[0m\x1b[1m  {:?}\x1b[0m", args.palette);
    match args.dither {
        Dither::Bayer2 | Dither::Bayer4 | Dither::Bayer8 | Dither::Bayer16
            => println!("   \x1b[33;1mDither:\x1b[0m\x1b[1m   {:?} - {:?}\x1b[0m", args.dither, args.bayer.unwrap_or(32)),
        Dither::None => println!("   \x1b[33;1mDither:\x1b[0m\x1b[1m   {:?}\x1b[0m", args.dither)
    }
    println!("\n   \x1b[32;1mInput:\x1b[0m    {}", args.input.to_str().unwrap());
    println!("   \x1b[33;1mOutput:\x1b[0m   {}", output.to_str().unwrap());
    if args.runtime {
        println!("\n   \x1b[33;1mRuntime:\x1b[0m  {}", args.runtime);
    }
    if args.size.is_some(){
        println!("   \x1b[33;1mPreview:\x1b[0m  {}x{} px", args.size.unwrap() * 2, args.size.unwrap());
    }
    print_dashes(&args.size);
}

pub fn print_measurements(load: Duration, proc: Duration, save: Duration) {
    println!(" \x1b[1mPerformance:\x1b[0m");
    println!("   \x1b[32;1mTotal:\x1b[0m \x1b[1m  {:>8.1?}\x1b[0m", load + proc + save);
    println!("   \x1b[33;1mLoad:\x1b[0m    {:>8.1?}", load);
    println!("   \x1b[33;1mProcess:\x1b[0m {:>8.1?}", proc);
    println!("   \x1b[33;1mSave:\x1b[0m    {:>8.1?}", save);
}

pub fn print_preview(img: DynamicImage, rows: u32) {
    let width = img.width();
    let height = img.height();

    let cols = rows * 2;
    let step_x = (width / cols).max(1);
    let step_y = (height / rows).max(1);

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