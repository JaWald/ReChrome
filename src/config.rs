use crate::cli::*;
use crate::cli::Args;
use crate::cli::Dither::*;
use crate::cli::Palette::*;
use crate::error::AppError;
use crate::data::*;

pub struct Config {
    pub input: String,
    pub output: String,
    //pub format: String,

    pub palette: Vec<[u8; 3]>,
    pub palette_name: String,
    pub dither: Dither,
    pub ampl: f32,

    pub size: u32,
    pub runtime: bool,
    pub test: TestType,
}

pub fn from_args(args: &Args) -> Result<Config, AppError> {
    let palette = match &args.palette {
        Some(pal) => {
            match pal {
                Everforest => EVERFOREST.to_vec(),
                Gruvbox => GRUVBOX.to_vec(),
                Kanagawa => KANAGAWA.to_vec(),
                Molokai => MOLOKAI.to_vec(),
                Papercut => PAPERCUT.to_vec(),
                Solarized => SOLARIZED.to_vec()
            }
        }
        None => vec![]
    };
    let palette_name = match &args.palette {
        Some(pal) => {
            match pal {
                Everforest => "Everforest",
                Gruvbox => "Gruvbox",
                Kanagawa => "Kanagawa",
                Molokai => "Molokai",
                Papercut => "Papercut",
                Solarized => "Solarized"
            }
        }
        None => ""
    }.to_string();
    let dither = args.dither;
    let ideal_ampl = match dither {
        Raw => 0,
        Bayer2 => 8,
        Bayer4 => 16,
        Bayer8 => 32,
        Bayer16 => 64
    };
    let ampl = args.ampl.unwrap_or_else(|| ideal_ampl) as f32;

    let input = match args.input.to_str() {
        Some(input) => input,
        None => return Err(AppError::InputFileDoesNotExist(args.input.clone()))
    }.to_string();
    let format = match args.format {
        Format::Png => "png".to_string(),
        Format::Jpg => "jpg".to_string(),
        Format::Jpeg => "jpeg".to_string()
    };
    let output = create_output_path(&args, palette_name.as_str(), ampl, dither, format.as_str())?;

    let size= args.size.unwrap_or(0);
    let runtime = args.runtime;
    let test = match args.test {
        Some(test) => {test}
        None => TestType::None
    };
    Ok(Config { input, output, /*format,*/ palette, palette_name, dither, ampl, size, runtime, test })
}

// sets output path either to user desired path OR creates a new one from given arguments
// format: <originalName>_<palette>_[<dither>]-[<amplitude>].<format>
pub fn create_output_path(args: &Args, palette_name: &str, ampl: f32, dither: Dither, format: &str) -> Result<String, AppError> {
    match &args.output {
        Some(path) => Ok(path.to_string_lossy().to_string()),
        None => {
            let mut path = args.input.clone();
            let input_stem = path.file_stem()
                .ok_or_else(|| AppError::InputFileDoesNotExist(args.input.clone()))?
                .to_string_lossy();
            let palette_str = match palette_name {
                "Everforest" => "evrfrst",
                "Gruvbox"    => "gruvbox",
                "Kanagawa"   => "kanagwa",
                "Molokai"    => "molokai",
                "Papercut"   => "paprcut",
                "Solarized"  => "solrizd",
                &_ => "",
            };
            let ampl_str = match ampl {
                2.0   => "--2",
                16.0  => "-16",
                32.0  => "-32",
                64.0  => "-64",
                128.0 => "128",
                _ => ""
            };
            let dither_str = match dither {
                Raw => "".to_string(),
                Bayer2 =>  format!("_bayer-2-{}", ampl_str),
                Bayer4 =>  format!("_bayer-4-{}", ampl_str),
                Bayer8 =>  format!("_bayer-8-{}", ampl_str),
                Bayer16 => format!("_bayer16-{}", ampl_str)
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