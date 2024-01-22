#[allow(dead_code)]

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RgbTransform{
    r: (f32, f32, f32, f32),
    b: (f32, f32, f32, f32),
    g: (f32, f32, f32, f32),
    a: (f32, f32, f32, f32),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum ColorRange{
    #[default]
    Undefined,
    Rgb(u8),
    Rgba(u8),
    Luma(u8),
    LumaAlpha(u8),
    // RgbTransform(RgbTransform, u8),
}

impl ColorRange{
    pub fn get_precision(&self) -> u8{
        match self{
            Self::Rgb(u) => *u,
            Self::Rgba(u) => *u,
            Self::Luma(u) => *u,
            Self::LumaAlpha(u) => *u,
            Self::Undefined => 8,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, )]
pub enum Output{
    #[default]
    Block,
    Ascii,
    Braille,
    Single(char),
}
