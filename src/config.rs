use crate::cli::*;
use crate::cli::Args;
use crate::cli::Dither::*;
use crate::cli::Palette::*;
use crate::error::AppError;
use crate::palettes::*;

pub struct Config {
    pub input: String,
    pub output: String,
    //pub format: String,

    pub palette: Vec<[u8; 3]>,
    pub palette_print: Palette,
    pub dither: Dither,
    pub ampl: f32,

    pub size: u32,
    pub runtime: bool,
    pub test: TestType,
}


pub fn from_args(args: &Args) -> Result<Config, AppError> {
    let palette = match args.palette {
        Everforest => EVERFOREST.to_vec(),
        Gruvbox => GRUVBOX.to_vec(),
        Kanagawa => KANAGAWA.to_vec(),
        Molokai => MOLOKAI.to_vec(),
        Papercut => PAPERCUT.to_vec(),
        Solarized => SOLARIZED.to_vec(),
    };
    let palette_print = args.palette.clone();
    let dither = args.dither;
    let ideal_ampl = match dither {
        Raw => 0,
        Bayer2 => 8,
        Bayer4 => 16,
        Bayer8 => 32,
        Bayer16 => 64
    };
    let ampl = args.bayer.unwrap_or_else(|| ideal_ampl) as f32;

    let input = match args.input.to_str() {
        Some(input) => input,
        None => return Err(AppError::InputFileDoesNotExist(args.input.clone()))
    }.to_string();
    let format = match args.format {
        Format::Png => "png".to_string(),
        Format::Jpg => "jpg".to_string(),
        Format::Jpeg => "jpeg".to_string()
    };
    let output = create_output_path(&args, ampl, dither, format.as_str())?;

    let size= args.size.unwrap_or(0);
    let runtime = args.runtime;
    let test = match args.test {
        Some(test) => {test}
        None => TestType::None
    };
    Ok(Config { input, output, /*format,*/ palette, palette_print, dither, ampl, size, runtime, test })
}

// sets output path either to user desired path OR creates a new one from given arguments
pub fn create_output_path(args: &Args, ampl: f32, dither: Dither, format: &str) -> Result<String, AppError> {
    match &args.output {
        Some(path) => Ok(path.to_string_lossy().to_string()),
        None => {
            let mut path = args.input.clone();
            let input_stem = path.file_stem()
                .ok_or_else(|| AppError::InputFileDoesNotExist(args.input.clone()))?
                .to_string_lossy();
            let palette_str = match args.palette {
                Everforest => "everforest",
                Gruvbox => "gruvbox",
                Kanagawa => "kanagawa",
                Molokai => "molokai",
                Papercut => "papercut",
                Solarized => "solarized"
            };
            let dither_str = match dither {
                Raw => "".to_string(),
                Bayer2 => format!("_bayer2-{}", ampl),
                Bayer4 => format!("_bayer4-{}", ampl),
                Bayer8 => format!("_bayer8-{}", ampl),
                Bayer16 => format!("_bayer16-{}", ampl)
            };
            let file = format!(
                "{}_{}{}.{}",
                input_stem,
                palette_str,
                dither_str,
                format
            );
            path.set_file_name(file);
            Ok(path.to_string_lossy().to_string())
        }
    }
}