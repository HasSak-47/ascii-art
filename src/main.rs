mod edges;

use image::{self, GenericImageView};
use std::io::prelude::Write;
use std::{env, default};

mod parser;
mod parameters;
use crate::parser::parse;
use crate::parameters::*;

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct RgbTransform{
    r: (f32, f32, f32, f32),
    b: (f32, f32, f32, f32),
    g: (f32, f32, f32, f32),
    a: (f32, f32, f32, f32),
}

#[derive(Debug, Clone, Default)]
pub enum ColorRange{
    #[default]
    Undefined,
    Rgb(u8),
    Rgba(u8),
    Luma(u8),
    LumaAlpha(u8),
    RgbTransform(RgbTransform),
}

#[derive(Default, Debug, Clone)]
pub struct Options{
    in_path    : String,
    out_path   : String,
    size       : (u32, u32),
    color_range: ColorRange,
}

fn locate(start: Option<usize>, end: usize, mat: char, data: &Vec<char>) -> Option<(usize, usize)> {
    let start = start?;

    for i in start..end{
        if mat == data[i]{
            return Some((start, i));
        }
    }

    Some((start, end))
}


fn parse_option<S : AsRef<str>>(arg: S) -> Options{
    let arg : &str = arg.as_ref();

    // sets default options
    let mut opt = Options::default();
    opt.in_path = arg.to_string();
    opt.out_path = "out.".to_string() + arg;
    opt.size =  (44, 0);

    let opt_div= arg.find(':');
    // No options passed
    if opt_div.is_none(){
        return opt
    }
    let s_chars : Vec<char> = arg.chars().collect();
    let s_chars_len = s_chars.len();

    let in_path     = locate(Some(0), s_chars_len, ':', &s_chars);
    let out_path    = locate(arg.find("out=")  , s_chars_len, ',', &s_chars);
    let size        = locate(arg.find("size=") , s_chars_len, ',', &s_chars);
    let color_range = locate(arg.find("color="), s_chars_len, ',', &s_chars);

    if let Some(in_path) = in_path
    { opt.in_path = parse(&arg[in_path.0..in_path.1]) }

    if let Some(out_path) = out_path
    { opt.out_path = parse(&arg[out_path.0 + 4..out_path.1]) }

    if let Some(size) = size
    { opt.size = parse(&arg[size.0 + 5..size.1]) }

    if let Some(color_range) = color_range
    { opt.color_range = parse(&arg[color_range.0 + 6..color_range.1]) }

    opt
}

fn write_rgb_color(p: u32, c: (u8, u8, u8)) -> String{
    format!("\x1b[48;2;{};{};{}m", c.0, c.1, c.2)
}

fn write_rgba_color(p: u32, c: (u8, u8, u8)) -> String{
    format!("\x1b[38;2;{};{};{}m", c.0, c.1, c.2)
}

fn process_rgb(p: u32, img: image::RgbImage) -> String{
    let p = p as u8;
    let mut r = String::new();
    let mut latest_color = (0, 0, 0);
    for row in img.rows(){
        for pixel in row{
            let current_color = ( pixel.0[0], pixel.0[1], pixel.0[2],);

            if current_color != latest_color{
                latest_color = current_color;
                r += &write_rgb_color(p as u32, current_color);
            }
            r +=  " " ;
        }
        r += "\n";
    }
    latest_color = (0, 0, 0);
    r += &write_rgb_color(p as u32, latest_color);

    r
}

fn process_rgba(p: u32, img: image::RgbaImage) -> String{
    let p = p as u8;
    let mut r = String::new();
    let mut latest_color = (0, 0, 0);
    for row in img.rows(){
        for pixel in row{
            let current_color = (pixel.0[0], pixel.0[1], pixel.0[2]);

            if current_color != latest_color{
                latest_color = current_color;
                r += &write_rgba_color(p as u32, current_color);
            }
            r += if pixel.0[3] != 0 {"#"} else { " " };
        }
        println!();
        r += "\n";
    }
    latest_color = (0, 0, 0);
    r += &write_rgb_color(p as u32, latest_color);

    r
}


fn process_image(mut opts: Options) {
    let img = image::open(&opts.in_path).expect("file not found!");
    let dims = img.dimensions();
    let ratio = dims.1 as f32 / dims.0 as f32;
    opts.color_range = match img.color(){
        image::ColorType::Rgb8 => ColorRange::Rgb(8),
        image::ColorType::Rgba8 => ColorRange::Rgba(8),
        _ => {ColorRange::LumaAlpha(0)},
    };
    println!("{opts:?}");
    

    let resized = if opts.size.1 == 0 {
        let w = opts.size.0 * 2;
        let h = opts.size.0 as f32 * ratio;
        img.resize_exact(w, h as u32, image::imageops::Nearest)
    }
    else{
        img.resize_exact(opts.size.0, opts.size.1, image::imageops::Nearest)
    };

    let img_ascii = match opts.color_range{
        ColorRange::Rgba(p) => process_rgba(p as u32, resized.into_rgba8()),
        _ => process_rgb(16, resized.into_rgb8()),
    };

    println!("{img_ascii}");
    let mut file = std::fs::File::create(opts.out_path).unwrap();
    file.write(img_ascii.as_bytes()).unwrap();

}

fn main() {
    let mut args : Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("usage: ascii-art file[:opt=val,opt] file[...] ...");
        return;
    }

    args.remove(0);
    let options : Vec<Options> = args.iter().map(|x| parse_option(x)).collect();
    options.iter().for_each(|x| process_image(x.clone()));
}
