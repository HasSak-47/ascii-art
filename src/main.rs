mod edges;

use image::{DynamicImage, ImageBuffer, Pixel};
use image::{self, GenericImageView};
use std::env;

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

fn cmp_pixels(a: image::Rgba<u8>, b: image::Rgba<u8>, range: ColorRange) -> bool{
    use ColorRange as RG;
    match range{
        RG::Luma(_) => a.to_luma() == b.to_luma(),
        RG::Rgb(_) => a.to_rgb() == b.to_rgb(),

        _ => a == b,
    }
}

static NONE_NONE : image::Rgba<u8> = image::Rgba([0,0,0,0]);
static NONE_FULL : image::Rgba<u8> = image::Rgba([0,0,0,255]);

fn handle_luma(current_color: image::Rgba<u8>, prev_color: image::Rgba<u8>, range: u8) -> String {
    let luma_color = current_color.to_luma();
    let luma = (luma_color.0[0] * range) / range;
    format!("\x1b[38;2;{luma};{luma};{luma}m")
}

fn handle_char(p: image::Rgba<u8>, c: char) -> char{
    if p.0[0] != 0 && p.0[1] != 0 && p.0[2] != 0
        { c }
    else
        { ' ' }
}

fn process_image(mut opts: Options) {
    let img = image::open(&opts.in_path).expect("file not found!");
    let dims = img.dimensions();
    let ratio = dims.1 as f32 / dims.0 as f32;
    opts.color_range = match img.color(){
        image::ColorType::Rgb8 => ColorRange::Rgb(8),
        image::ColorType::Rgba8 => ColorRange::Rgba(8),
        image::ColorType::L8 => ColorRange::Luma(8),
        image::ColorType::La8 => ColorRange::LumaAlpha(8),
        _ => panic!("color yet not implemented!"),
    };
    if opts.size.1 == 0{
        opts.size.1 = (opts.size.0 as f32 * ratio) as u32;
    }
    match opts.out_type {
        Output::Braille => {
            opts.size.0 *= 2;
            opts.size.1 *= 4;
        }
        _ => {},
    }
    opts.size.0 *= 2;
    let img = img.resize_exact(opts.size.0, opts.size.1, image::imageops::FilterType::Nearest);
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
