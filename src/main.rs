use std::io::{self, prelude::*};

use std::str;

#[allow(unused_imports)]
use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Rgb, Rgba};

use std::env;

// esteganografia
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // let addr = read_all::<Ipv4Addr>(&args[1]);

    let nom_img = store_f_in_image(
        &args[1],
        &args[2]
    ).unwrap();

    read_f_in_image(
        &nom_img,
        "esto_es_prueba.txt"
    )

    // read_bin(
    //     BinaryReader::new(&args[2], OpenType::Open)
    //         .expect("Failed to create reader")
    // )

    // Ok(())
}

fn store_f_in_image(img_path: &str, file_path: &str) -> io::Result<String> {
    let mut file: std::fs::File = std::fs::File::open(file_path)?;

    let chunk_size = 0x4000;

    let mut img: image::DynamicImage = match image::open(img_path) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    };

    let mut fpos_x : u32 = 0;
    let mut fpos_y : u32 = 0;

    loop {
        let mut chunk = Vec::with_capacity(chunk_size);
        let n = std::io::Read::by_ref(&mut file)
            .take(chunk_size as u64)
            .read_to_end(&mut chunk)?;
        if n == 0 { break; }
        // let aa = String::from_utf8(chunk).unwrap();
        // let bytes : String = format!("{:#b}", aa ); //.into_iter().collect());
        println!("{:?}", chunk);
        for a in chunk {

            print!("{:?} \t", a);

            // let bytes = format!("{:#b}", a);
            // for aa in bytes.chars() {
            //     print!("-{}", aa);
            // }
            let mut pixel: Rgba<u8> = img.get_pixel(fpos_x , fpos_y);
            pixel = image::Rgba([pixel[0], pixel[1], pixel[2], a]);
            img.put_pixel(fpos_x,fpos_y,pixel);


            if fpos_x > img.width() {
                fpos_x = 0;
                fpos_y += 1;

            } else {
                fpos_x += 1;

            }

            println!();
            // println!("{:#b}", a);
            // println!("{:?}", bs);
        }

        println!("------------------");

        if n < chunk_size { break; }
    }

    let nombre = format!("copia_esteg_{}", img_path);
    img.save(&nombre).unwrap();
    Ok(nombre)

}

fn read_f_in_image(img_path: &str, file_path: &str) -> io::Result<()> {
    let mut _file: std::fs::File = std::fs::File::create(file_path)?;

    let img: image::DynamicImage = match image::open(img_path) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    };

    let rango = [0..img.width()];

    for _y in 0..img.height() {
        // for y in 0..img.height() {
        //     let mut pixel: Rgba<u8> = img.get_pixel(x,y); // .get_pixel_mut(x, y);
        //     let mut pixel: Rgba<u8> = img.get_pixel(x,y); // .get_pixel_mut(x, y);
        //     // pixel = image::Rgba([pixel[0], pixel[1], pixel[2], 0]);
        //     file.write(&pixel[3]);
        // }

// : Vec<u32> 
        let aa = rango
                .iter()
                .map (
                    |x|
                    // println!("{:?}", x);
                    img.get_pixel(*x,y)[3]
                )
                // .collect()
                ;
                
        // str::from_utf8( &aa ).unwrap();

        println!("{:?}", aa);

        // file.write(
        // )
    }

    Ok(())

}

// Loads an entire file of ip addresses as a Vector of Result<Ipv4Addr> structs
// fn read_all<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
//     std::fs::read_to_string(file_name)
//         .expect("file not found!")
//         .lines()
//         .map(|x| x.parse())
//         .collect()
// }

// fn read_bin(mut bin:BinaryReader) {
//     let read_value = bin.read_bytes().expect("Failed to write f32");
//     println!("{:?}", read_value);
// }

#[allow(dead_code)]
fn read_image(img_path: &str)  {
    println!("imagen : {}", img_path);

    let mut img: image::DynamicImage = match image::open(img_path) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    };

    // for (x, y, mut pixel) in img.pixels() {
    //     println!("pixel: {} {}", x, y );
    //     let r = (0.3 * x as f32) as u8;
    //     let b = (0.3 * y as f32) as u8;

    //     println!("{:?}", pixel);

    //     *pixel = image::Rgba([r, 0, b, 0]);
    // }

    for x in 0..img.width() {
        for y in 0..img.height() {
            let mut pixel: Rgba<u8> = img.get_pixel(x,y); // .get_pixel_mut(x, y);
            pixel = image::Rgba([pixel[0], pixel[1], pixel[2], 0]);
            img.put_pixel(x,y,pixel);
        }
    }

    img.save(format!("copia_{}", img_path)).unwrap();
}
