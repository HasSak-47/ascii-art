use crate::{Options, Output};
use crate::parameters::*;

pub trait Parse where Self: Sized{
    fn parse(s: &str) -> Self;
}

impl Parse for String{
    fn parse(s: &str) -> Self { s.to_string() }
}

impl Parse for (u32, u32){
    fn parse(s: &str) -> Self {
        let x_pos = s.find('x');
        if let Some(p) = x_pos {
            (
                u32::from_str_radix(&s[0..p], 10).unwrap(),
                u32::from_str_radix(&s[p + 1..s.len()], 10).unwrap()
            )
        }
        else{
            (u32::from_str_radix(s, 10).unwrap(), 0u32)
        }
    }
}

impl Parse for ColorRange{
    fn parse(s: &str) -> Self {
        let precision= s.find('_');
        let range_t = if let Some(p) = s.find('_'){
            &s[0..p]
        }
        else{s};

        if precision.is_none() {
            if "rgb" == range_t { return ColorRange::Rgb(8);}
            if "rgba" == range_t { return ColorRange::Rgba(8);}
            if "luma" == range_t { return ColorRange::Luma(8);}
            if "luma-alpha" == range_t { return ColorRange::LumaAlpha(8);}
        }

        ColorRange::default()
    }
}

impl Parse for Output{
    fn parse(s: &str) -> Self {
        if s == "ascii" { return Output::Ascii; }
        else if s == "braille" { return Output::Braille; }
        else if s == "block"{ return Output::Block; }

        let last = s.chars().nth(s.len() - 1).unwrap();
        Output::Single(last)
    }
}

pub fn parse<P: Parse>(s: &str) -> P{ P::parse(s) }

fn locate(start: Option<usize>, end: usize, mat: char, data: &Vec<char>) -> Option<(usize, usize)> {
    let start = start?;

    for i in start..end{
        if mat == data[i]{
            return Some((start, i));
        }
    }

    Some((start, end))
}

pub fn parse_option<S : AsRef<str>>(arg: S) -> Options{
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
    let out_type    = locate(arg.find("type="), s_chars_len, ',', &s_chars);


    if let Some(in_path) = in_path
    { opt.in_path = parse(&arg[in_path.0..in_path.1]) }

    if let Some(out_path) = out_path
    { opt.out_path = parse(&arg[out_path.0 + 4..out_path.1]) }

    if let Some(size) = size
    { opt.size = parse(&arg[size.0 + 5..size.1]) }

    if let Some(color_range) = color_range
    { opt.color_range = parse(&arg[color_range.0 + 6..color_range.1]) }

    if let Some(out_type) = out_type
    { opt.out_type = parse(&arg[out_type.0 + 5..out_type.1]) }

    opt
}


