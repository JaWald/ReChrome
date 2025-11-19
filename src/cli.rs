use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version="0.0.1", about, long_about = None)]
pub struct Args {
    /// Input file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// Color palette
    #[arg(short, long, help="Available: \n   > Gruvbox\n   > Gameboy")]
    pub palette: String,

    /// Output file path (optional)
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}