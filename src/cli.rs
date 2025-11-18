use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// Color palette
    #[arg(short, long)]
    pub palette: String,

    /// Output file path
    #[arg(short, long, default_value = "content")]
    pub output: PathBuf,
}