mod edges;

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
    let height = 22;
    let width  = height * 2;

    let image = img.resize_exact(width * 2, height * 3, image::imageops::Nearest).into_luma8();

    // let mut char_map = [[' '; 100]; 100];

    let mut file = std::fs::File::create("tests/test.txt").unwrap();
    let mut write = "\n".to_string();
    for j in 0..height{
        for i in 0..width{
            let mut char_id : usize = 0;
            for jd in 0..3u32{
                for id in 0..2u32{
                    let val : usize = 1 << (jd + id * 3) as usize;
                    let valid = image.get_pixel(i * 2 + id, j * 3 + jd).0[0] >= 1;
                    char_id |= val * valid as usize;
                }
            }
            // print!(" {:02x}", char_id);
            write.push(edges::CHARACTERS[char_id]);
        }
        write.push('\n');
    }

    // println!("{}", write);
    file.write(write.as_bytes()).unwrap();
}
