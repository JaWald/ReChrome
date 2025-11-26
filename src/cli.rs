use std::cmp::max;
use std::fmt::Debug;
use std::path::PathBuf;
use std::time::Duration;
use clap::{Parser, ValueEnum};
use image::{DynamicImage, GenericImageView};
use crate::config::Config;

#[derive(Parser, Debug)]
#[command(version="0.0.1", about, long_about = None)]
pub struct Args {
    /// Color palette
    #[arg(short, long, value_enum, help = "\x1b[33;1mColor Palette   \x1b[0m")]
    pub palette: Palette,

    /// Dithering Modes
    #[arg(short, long, value_enum, default_value = "bayer16", help = "Dithering Modes ")]
    pub dither: Dither,

    /// Bayer intensity
    #[arg(short, long, help = "Bayer amplitude  [default: bayerX * 4] \n")]
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

    #[arg(short, long)]
    pub test: Option<TestType>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Palette {
    Gruvbox,
    Everforest,
    Kanagawa,
    Molokai,
    Papercut,
    Solarized
}

#[derive(Debug, Clone, ValueEnum, Copy)]
pub enum Dither {
    Raw,
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
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TestType {
    None,
    Palette,
    Dither,
    Amplitude,
    All,
}

// prints dashes in corresponding size to preview image
pub fn print_dashes(size: u32) {
    println!("{}", "-".repeat(max(80usize, ((size + 1) * 4) as usize)));
}

pub fn print_selection(config: &Config, output: &str) {
    print_dashes(config.size);
    println!(" \x1b[1mSelection: \x1b[0m");
    println!("   \x1b[32;1mPalette:\x1b[0m\x1b[1m  {:?}\x1b[0m", config.palette_print);
    match config.dither {
        Dither::Bayer2 |
        Dither::Bayer4 |
        Dither::Bayer8 |
        Dither::Bayer16 => println!("   \x1b[33;1mDither:\x1b[0m\x1b[1m   {:?} - {:?}\x1b[0m", config.dither, config.ampl),
        Dither::Raw => println!("   \x1b[33;1mDither:\x1b[0m\x1b[1m   {:?}\x1b[0m", config.dither)
    }
    println!("\n   \x1b[32;1mInput:\x1b[0m    {}", config.input);
    println!("   \x1b[33;1mOutput:\x1b[0m   {}", output);
    if config.runtime {
        println!("\n   \x1b[33;1mRuntime:\x1b[0m  {}", config.runtime);
    }
    if config.size > 0{
        println!("   \x1b[33;1mPreview:\x1b[0m  {}x{} px", config.size * 2, config.size);
    }
    print_dashes(config.size);
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