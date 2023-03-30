mod edges;

use image;
use std::io::prelude::Write;
use std::env;
use std::path::Path;

fn make_art<S: AsRef<Path> + ToString>(in_path: S) -> String {
    let img = image::open(&in_path).unwrap();
    let height = 22;
    let width = height * 2;
    let image = img.resize_exact(width * 2, height * 3, image::imageops::Nearest).into_rgb8();
    let out_path = in_path.to_string() + ".txt";
    let out_file = std::fs::File::create("out_path").unwrap();

    let mut out = String::new();

    for j in 0..height{
        for i in 0..width{

        }
    }


    out
}

fn main() {
    let mut args : Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("usage [name] file1 file2 file3");
        return;
    }

    args.remove(0);

    let img = match image::open(args[0].clone()){
        Ok(k) => k,
        Err(_) => {
            println!("file {args:?} not found");
            return;
        }
    };
    let height = 22;
    let width  = height * 2;
    let image = img.resize_exact(width * 2, height * 3, image::imageops::Nearest).into_luma8();

    // let mut char_map = [[' '; 100]; 100];
    let mut file = std::fs::File::create(format!("{}.txt", args[0])).unwrap();
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
