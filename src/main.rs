use image;
use std::io::prelude::Write;

struct Vector<T: Default>{
    _vals : [T; 2],
}

// so much boiler plateeeeeeeeeeee
impl<T> Vector<T> where
T: Default
{
    fn new() -> Self{Vector {_vals :[T::default(), T::default()]}}
    fn     x(&self    ) -> &T    {&self._vals[0]}
    fn x_mut(&mut self) -> &mut T{&mut self._vals[0]}
    fn     y(&self    ) -> &T    {&self._vals[1]}
    fn y_mut(&mut self) -> &mut T{&mut self._vals[1]}
}


struct Geometry{
    pub points: Vector<u32>,
}

fn main() {
    let mut img = image::open("test.png").unwrap();
    let image = img.resize_exact(100, 50, image::imageops::Nearest).into_luma8();

    // let mut char_map = [[' '; 100]; 100];

    let intensities = [' ', '.', ':', ';', '+', '=', '$', '&'];
    let slopes = [
        '-', // 0.0
        '/', // 60.0
        '|', // 90.0
        '\\',// -60.0
    ];

    let mut file = std::fs::File::create("tests/test.txt").unwrap();
    let mut write = String::new();
    for j in 0..50{
        for i in 0..100{
            println!("i: {} j: {}", i, j);
            let inten = (image.get_pixel(i, j).0[0] as f32 * 8.0 / 255.0) as usize;

            write.push(intensities[inten as usize]);
        }
        write.push('\n');
    }

    file.write(write.as_bytes()).unwrap();
}
