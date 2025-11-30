use std::fmt::Debug;
use std::path::PathBuf;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version="0.0.1", about, long_about = None)]
pub struct Args {
    /// Color palette
    #[arg(short, long, value_enum, help = "\x1b[33;1mColor Palette   \x1b[0m")]
    #[clap(required_unless_present = "test")]
    pub palette: Option<Palette>,

    /// Dithering Modes
    #[arg(short, long, value_enum, default_value = "bayer8", help = "Dithering Modes ")]
    pub dither: Dither,

    /// Bayer amplitude
    #[arg(short, long, help = "Bayer Amplitude  [default: bayerX * 4, <256] \n")]
    pub ampl: Option<u8>,

    /// Input  file path
    #[arg(short, long, help = "\x1b[33;1mInput file path  \x1b[0m")]
    pub input: PathBuf,

    /// Output file type
    #[arg(short, long, value_enum, default_value = "jpg")]
    pub format: Format,

    /// Output file path
    #[arg(short, long, help = "Output file path [default: <input>_<palette>_<dither>.<format>]\n")]
    pub output: Option<PathBuf>,

    /// Show in-terminal (recommended value: <= 20)
    #[arg(short = 's', long = "showcase")]
    pub size: Option<u32>,

    /// Show runtime
    #[arg(short = 'r', long, help = "Show runtime performance\n")]
    pub runtime: bool,

    /// Tests arguments
    #[arg(short, long, help = "\x1b[33;1mTest arguments  \x1b[0m")]
    pub test: Option<TestType>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Palette {
    Atomone,
    Catppuccin,
    Darcula,
    Everforest,
    Gruvbox,
    Kanagawa,
    Monokai,
    Nord,
    Papercut,
    Solarized,
    Synthwave
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
#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum TestType {
    None,
    Palette,
    Dither,
    Amplitude,
    All,
}