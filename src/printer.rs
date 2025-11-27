use std::cmp::max;
use std::time::Duration;
use image::{DynamicImage, GenericImageView};
use crate::cli::Dither;
use crate::config::Config;

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

pub fn print_measurements(size: u32, load: Duration, proc: Duration, save: Duration) {
    println!(" \x1b[1mPerformance:\x1b[0m");
    println!("   \x1b[32;1mTotal:\x1b[0m \x1b[1m  {:>8.1?}\x1b[0m", load + proc + save);
    println!("   \x1b[33;1mLoad:\x1b[0m    {:>8.1?}", load);
    println!("   \x1b[33;1mProcess:\x1b[0m {:>8.1?}", proc);
    println!("   \x1b[33;1mSave:\x1b[0m    {:>8.1?}", save);
    print_dashes(size);
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
    print_dashes(rows);
}

// for palette conversion in development
pub fn _print_palette(str: &String) {
    let content = std::fs::read_to_string(str).expect("Should have been able to read file");

    let mut count = 0;
    for l in content.lines() {
        if l.starts_with("//") || l.is_empty() { continue }
        let r = u8::from_str_radix(&l[0..2], 16).expect("Should have been able to parse red");
        let g = u8::from_str_radix(&l[2..4], 16).expect("Should have been able to parse green");
        let b = u8::from_str_radix(&l[4..6], 16).expect("Should have been able to parse blue");
        print!("[{:>3}, {:>3}, {:>3}], ", r, g, b);
        if count % 4 == 3 {
            println!();
        }
        count += 1;
    }
    println!();
}

// Mpre(i,j) = Mint(i,j) / n^2 - 0.5 * maxValue
pub fn _print_matrix(matrix: [[u8; 16]; 16]) {
    let mut max = 0.0;
    for line in matrix {
        for num in line {
            let new = num as f32 / (matrix.len() as f32 * matrix.len() as f32);
            if new > max {
                max = new;
            }
        }
    }
    println!("MAX = {}", max);

    print!("[");
    for line in matrix {
        print!("[");
        for num in line {
            let new = num as f32 / (matrix.len() as f32 * matrix.len() as f32);
            print!(" {:>4.7},", new - 0.5 * max);
        }
        println!("],")
    }
    println!("];");
}