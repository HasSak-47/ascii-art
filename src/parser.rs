use crate::ColorRange;

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
        println!("{s}");
        let presicion= s.find('_');
        let range_t = if let Some(p) = s.find('_'){
            &s[0..p]
        }
        else{s};

        if presicion.is_none() {
            if "rgb" == range_t { return ColorRange::Rgb(8);}
            if "rgba" == range_t { return ColorRange::Rgba(8);}
            if "luma" == range_t { return ColorRange::Luma(8);}
            if "luma-alpha" == range_t { return ColorRange::LumaAlpha(8);}
        }

        ColorRange::default()
    }
}

pub fn parse<P: Parse>(s: &str) -> P{ P::parse(s) }
