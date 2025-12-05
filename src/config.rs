use crate::cli::*;
use crate::cli::Args;
use crate::cli::Dither::*;
use crate::cli::Palette::*;
use crate::data;
use crate::error::AppError;
use crate::data::*;

pub struct Config {
    pub input: String,
    pub output: String,
    pub format: Format,
    pub quality: u8,

    pub palette: data::Palette,
    pub dither: Dither,
    pub ampl: f32,

    pub size: u32,
    pub runtime: bool,
    pub test: TestType,
}

pub fn from_args(args: &Args) -> Result<Config, AppError> {
    let palette = match &args.palette {
        Some(p) => {
            match p {
                Atomone     => ATOMONE,
                Catppuccin  => CATPPUCCIN,
                Darcula     => DARCULA,
                Everforest  => EVERFOREST,
                Gruvbox     => GRUVBOX,
                Kanagawa    => KANAGAWA,
                Monokai     => MONOKAI,
                Nord        => NORD,
                Papercolor  => PAPERCOLOR,
                Solarized   => SOLARIZED,
                Synthwave   => SYNTHWAVE
            }
        }
        None => ATOMONE
    };
    let dither = args.dither;
    let ideal_ampl = match dither {
        Raw     => 0,
        Bayer2  => 8,
        Bayer4  => 16,
        Bayer8  => 32,
        Bayer16 => 64,
        Fs      => 16
    };
    let ampl = args.ampl.unwrap_or_else(|| ideal_ampl) as f32;

    let format = args.format.clone();
    let quality = args.quality.unwrap_or_else(|| args.quality.unwrap_or(90));
    let input = match args.input.to_str() {
        Some(input) => input,
        None => return Err(AppError::InputFileDoesNotExist(args.input.clone()))
    }.to_string();
    let output = create_output_path(&args, "", &palette, ampl, dither, &format)?;

    let size= args.size.unwrap_or(0);
    let runtime = args.runtime;
    let test = args.test.unwrap_or_else(|| TestType::None);
    Ok(Config { input, output, format, quality, palette, dither, ampl, size, runtime, test })
}

// sets output path either to user desired path OR creates a new one from given arguments
// format: <originalName>_<palette>_[<dither>]-[<amplitude>].<format>
pub fn create_output_path(args: &Args, test: &str, palette: &data::Palette, ampl: f32, dither: Dither, format: &Format) -> Result<String, AppError> {
    match &args.output {
        Some(path) => Ok(path.to_string_lossy().to_string()),
        None => {
            let mut path = args.input.clone();
            let input_stem = path.file_stem()
                .ok_or_else(|| AppError::InputFileDoesNotExist(args.input.clone()))?
                .to_string_lossy();
            let ampl_str = if ampl < 10.0 {
                format!("--{:.0}", ampl)
            } else if ampl < 100.0 {
                format!("-{:.0}", ampl)
            } else {
                format!("{:.0}", ampl)
            };
            let dither_str = match dither {
                Raw     => "".to_string(),
                Bayer2  =>  format!("_bayer-2-{}", ampl_str),
                Bayer4  =>  format!("_bayer-4-{}", ampl_str),
                Bayer8  =>  format!("_bayer-8-{}", ampl_str),
                Bayer16 => format!("_bayer16-{}", ampl_str),
                Fs      =>  format!("_floSt-{}", ampl_str),
            };
            let format_str = match format {
                Format::Png => "png".to_string(),
                Format::Jpeg => "jpeg".to_string(),
            };
            let file = format!(
                "{}{}_{}{}.{}",
                input_stem,
                test,
                palette.file_name,
                dither_str,
                format_str
            );
            path.set_file_name(file);
            Ok(path.to_string_lossy().to_string())
        }
    }
}