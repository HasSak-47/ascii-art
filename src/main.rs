mod edges;

use edges::{START, BRAILLE};
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
    let mut c = [pixel.0[0],pixel.0[1],pixel.0[2]];
    let p = opts.color_range.get_precision();
    let s = 255 / p;
    println!("{p} {c:?}:");
    for v in &mut c{
        let mv = *v as u32;
        let mv = (p as u32 * mv) as f32 / 255.;
        *v = s * mv.round() as u8; 
    }
    println!("{p} {c:?}");

    format!("{};{};{}m", c[0], c[1], c[2])
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

fn combine_pixels(pixels: &[image::Rgba<u8>; 6]) -> image::Rgba<u8>{
    let mut pixel = pixels[0].clone();

    pixel
}

fn handle_braille(opts: &Options, img: DynamicImage) -> String{
    let img = img.resize_exact(opts.size.0 * 2, opts.size.1 * 3, FilterType::Nearest);
    let mut buffer = String::with_capacity(opts.size.1 as usize * (opts.size.0 + 1) as usize);
    let mut last_pixels = combine_pixels(&[
        img.get_pixel(0, 0), img.get_pixel(1, 0), 
        img.get_pixel(0, 1), img.get_pixel(1, 1), 
        img.get_pixel(0, 2), img.get_pixel(1, 2), 
    ]);
    // buffer += &format!("\x1b[38;2;");
    // buffer += &handle_color(last_pixel, opts);
    for ij in 0..opts.size.0 * opts.size.1{
        let i = 2 * (ij % opts.size.0);
        let j = 3 * (ij / opts.size.0);
        let pixels = [
            img.get_pixel(i, j), img.get_pixel(i + 1, j), 
            img.get_pixel(i, j + 1), img.get_pixel(i + 1, j + 1), 
            img.get_pixel(i, j + 2), img.get_pixel(i + 1, j + 2), 
        ];
        let t = |x: u8| if x == 0 { 0u32 } else { 1u32 };
        let c =
              (t(pixels[0].0[3]) * 1)
            + (t(pixels[1].0[3]) * 8)
            + (t(pixels[2].0[3]) * 2)
            + (t(pixels[3].0[3]) * 16)
            + (t(pixels[4].0[3]) * 4)
            + (t(pixels[5].0[3]) * 32);

        let pixels = combine_pixels(&pixels);
        if pixels != last_pixels{
            last_pixels = pixels;
            buffer += &format!("\x1b[38;2;");
            buffer += &handle_color(last_pixels, opts);
        }
        buffer.push(BRAILLE[c as usize]);
        if (ij % opts.size.0) + 1 == opts.size.0 {
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
        Output::Braille => handle_braille(&opts, img),
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
