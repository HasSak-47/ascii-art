mod edges;

use image::{DynamicImage, Pixel, imageops::FilterType};
use image::{self, GenericImageView};
use std::env;
use std::fs::File;
use std::io::Write;

mod parser;
mod parameters;
use crate::parser::*;
use crate::parameters::*;


#[derive(Default, Debug, Clone)]
pub struct Options{
    in_path    : String,
    out_path   : String,
    size       : (u32, u32),
    color_range: ColorRange,
    out_type   : Output,
}

fn write_art(opts: Options, art: String) {
    let mut file = File::create(opts.out_path).unwrap();
    file.write_all(art.as_bytes()).unwrap();
}

fn handle_color(pixel: image::Rgba<u8>, opts: &Options) -> String{
    use ColorRange as CR;
    let pixel = match opts.color_range {
        CR::Luma(_) => {pixel.to_luma().to_rgba()},
        CR::LumaAlpha(_) => {pixel.to_luma_alpha().to_rgba()},
        _ => {pixel},
    };
    let c = (pixel.0[0],pixel.0[1],pixel.0[2],pixel.0[3]);
    format!("{};{};{}m", c.0, c.1, c.2)
}

fn handle_single(opts: &Options, img: DynamicImage, c: char) -> String{
    let img = img.resize_exact(opts.size.0, opts.size.1, FilterType::Nearest);
    let mut buffer = String::with_capacity(opts.size.1 as usize * (opts.size.0 + 1) as usize);
    let mut last_pixel = img.get_pixel(0, 0);
    buffer += &format!("\x1b[38;2;");
    buffer += &handle_color(last_pixel, opts);
    for ij in 0..opts.size.0 * opts.size.1{
        let i = ij % opts.size.0;
        let j = ij / opts.size.0;
        let pixel = img.get_pixel(i, j);
        if pixel != last_pixel{
            last_pixel = pixel;
            buffer += &format!("\x1b[38;2;");
            buffer += &handle_color(last_pixel, opts);
        }
        buffer.push(if pixel.0[3] == 0 {' '} else { c });
        if i + 1 == opts.size.0 {
            buffer += "\n";
        }
    }

    buffer
}

fn handle_block(opts: &Options, img: DynamicImage) -> String{
    let img = img.resize_exact(opts.size.0, opts.size.1, FilterType::Nearest);
    let mut buffer = String::with_capacity(opts.size.1 as usize * (opts.size.0 + 1) as usize);
    let mut last_pixel = img.get_pixel(0, 0);
    buffer += &format!("\x1b[48;2;");
    buffer += &handle_color(last_pixel, opts);
    for ij in 0..opts.size.0 * opts.size.1{
        let i = ij % opts.size.0;
        let j = ij / opts.size.0;
        let pixel = img.get_pixel(i, j);
        if pixel != last_pixel{
            last_pixel = pixel;
            buffer += &format!("\x1b[48;2;");
            buffer += &handle_color(last_pixel, opts);
        }
        buffer.push(' ');
        if i + 1 == opts.size.0 {
            buffer += "\n";
        }
    }

    buffer
}

fn handle_braille(opts: &Options, img: DynamicImage) -> String{
    let img = img.resize_exact(opts.size.0, opts.size.1, FilterType::Nearest);
    let mut buffer = String::with_capacity(opts.size.1 as usize * (opts.size.0 + 1) as usize);
    let mut last_pixel = img.get_pixel(0, 0);
    buffer += &format!("\x1b[48;2;");
    buffer += &handle_color(last_pixel, opts);
    for ij in 0..opts.size.0 * opts.size.1{
        let i = ij % opts.size.0;
        let j = ij / opts.size.0;
        let pixel = img.get_pixel(i, j);
        if pixel != last_pixel{
            last_pixel = pixel;
            buffer += &format!("\x1b[48;2;");
            buffer += &handle_color(last_pixel, opts);
        }
        buffer.push(' ');
        if i + 1 == opts.size.0 {
            buffer += "\n";
        }
    }

    buffer
}

fn process_image(mut opts: Options) {
    let img = image::open(&opts.in_path).expect("file not found!");
    let dims = img.dimensions();
    if opts.color_range == ColorRange::Undefined{
        opts.color_range = match img.color(){
            image::ColorType::Rgb8 => ColorRange::Rgb(8),
            image::ColorType::Rgba8 => ColorRange::Rgba(8),
            image::ColorType::L8 => ColorRange::Luma(8),
            image::ColorType::La8 => ColorRange::LumaAlpha(8),
            c => panic!("color {c:?} yet not implemented!"),
        }
    };

    let ratio = dims.1 as f32 / dims.0 as f32;
    if opts.size.1 == 0{
        opts.size.1 = (opts.size.0 as f32 * ratio) as u32;
    }
    println!("{opts:?}");
    /*
    match opts.out_type {
        Output::Braille => {
            opts.size.0 *= 2;
            opts.size.1 *= 4;
        }
        _ => {},
    }
    */
    opts.size.0 *= 2;

    let art = match opts.out_type{
        Output::Block => handle_block(&opts, img),
        Output::Single(c) => handle_single(&opts, img, c),
        _ => String::new(),
    };
    write_art(opts, art);
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
