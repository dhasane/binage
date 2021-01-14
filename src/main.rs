#[allow(unused_imports)]
use std::io::{self, prelude::*};

#[allow(unused_imports)]
use std::str;

#[allow(unused_imports)]
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Pixel, Rgb, Rgba};

#[allow(unused_imports)]
use std::env;

// use crate::color::{self, IntoColor};

// esteganografia
fn main() -> io::Result<()> {
    // let args: Vec<String> = env::args().collect();

    println!("{}", 161 | 0);

    // let nom_img = match store_f_in_image(&args[1], &args[2]) {
    //     Ok(a) => a,
    //     Err(err) => {
    //         eprintln!("error: {}", err);
    //         std::process::exit(1);
    //     }
    // };

    // println!("{}", nom_img);

    // read_f_in_image(&nom_img, "esto_es_prueba.txt")

    // print_image_pixels(&nom_img)

    // let aa : Vec<u32> = (0..50_u32)
    //     .map (|x| x + 1)
    //     .collect()
    //     ;
    // println!("{:?}", aa);

    Ok(())
}

fn modify_byte(byte: u8, pos: u8, max:u8, bit: u8) {

    // TODO: poner en el ultimo bit
    // seria chevere que tambien sirva para otras posiciones y no solo la ultima
    // pixel[pixel_pos] |= bit;
    algo asi como
        byte[8-max + pos] = bit

}

#[allow(dead_code)]
fn store_f_in_image(img_path: &str, file_path: &str) -> io::Result<String> {
    let mut file: std::fs::File = std::fs::File::open(file_path)?;

    let chunk_size = 0x4000;

    let mut img = DynamicImage::ImageRgba8(match image::open(img_path) {
        Ok(f) => f.into_rgba8(),
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    });

    // let rgba = open("path/to/some.png").unwrap().into_rgba();
    // let gray = DynamicImage::ImageRgba8(rgba).into_luma();

    // posiciones en x y y de la imagen
    let mut pos_x: u32 = 0;
    let mut pos_y: u32 = 0;
    // posicion dentro del pixel -> {0..3}
    let mut pixel_pos = 0;

    // cantidad de bits para ificar en el byte
    let pixel_internal_max = 2;
    // posicion que se esta modificando del byte
    let mut pixel_internal_pos = 0;


    // mientras haya elementos para leer, se meten en un chunk
    loop {
        let mut chunk = Vec::with_capacity(chunk_size);
        let n = std::io::Read::by_ref(&mut file)
            .take(chunk_size as u64)
            .read_to_end(&mut chunk)?;
        if n == 0 {
            break;
        }

        // para cada byte dentro del chunk
        for byte in chunk {
            println!("{:?} \t {}:{}", byte, pos_x, pos_y);

            // para cada bit dentro del byte
            for bit in format!("{:#b}", byte)[2..].chars() {
                println!("{}", bit);

                let mut pixel = img.get_pixel(pos_x, pos_y);
                println!("{:?}", pixel);

                modify_byte(
                    pixel[pixel_pos],
                    pixel_internal_pos,
                    pixel_internal_max,
                    bit as u8
                );

                pixel_pos += 1;
                if pixel_pos > 3 { pixel_pos = 0 }

                img.put_pixel(
                    pos_x, pos_y, // Rgba<u8>::from([pixel[0], pixel[1],pixel[2], byte])
                    pixel,
                );

                println!("{:?}", pixel);

                if pos_x > img.width() {
                    pos_x = 0;
                    pos_y += 1;
                } else {
                    pos_x += 1;
                }
            }

            // pixel[0] |= byte;
            // pixel[1] |= byte;
            // pixel[2] |= byte;

            println!();
        }

        println!("------------------");

        if n < chunk_size {
            break;
        }
    }

    println!("{}:{}", pos_x, pos_y);

    // for y in 0..pos_y + 1 {
    //     for x in 0..pos_x + 1 {
    //         let pixel = img.get_pixel(x, y);

    //         println!("{:?}", pixel);
    //     }
    // }
    // println!("------------------");
    // for y in 0..1 {
    //     // img.height() {
    //     for x in 0..50 {
    //         // img.width() {
    //         let pixel = img.get_pixel(x, y);
    //         println!("{:?}", pixel);
    //     }
    // }

    let nombre = format!("copia_esteg_{}", img_path);

    // let res = img.save(&nombre);
    Ok("hola".to_string())

    // Ok(match res {
    //     Ok(()) => nombre,
    //     Err(err) => {
    //         eprintln!("error: {}", err);
    //         std::process::exit(1);
    //     }
    // })
}

#[allow(dead_code)]
fn read_f_in_image(img_path: &str, file_path: &str) -> io::Result<()> {
    println!("leyendo {}", img_path);
    println!("creando {}", file_path);

    let mut file: std::fs::File = std::fs::File::create(file_path)?;

    let img: image::DynamicImage = match image::open(img_path) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    };

    println!(" tam imagen {}:{}", img.width(), img.height());

    for y in 0..img.height() {
        let aa: Vec<_> = (0..img.width())
            .map(|x| {
                println!("{}:{}  -> {:?}", x, y, img.get_pixel(x, y));
                img.get_pixel(x, y)[3]
            })
            .collect();

        println!("{:?}", aa);

        let pal = str::from_utf8(&aa).unwrap();
        println!("{}", pal);

        // println!("{:?}", aa);

        match file.write(&aa) {
            Ok(f) => f,
            Err(err) => {
                eprintln!("error: {}", err);
                std::process::exit(1);
            }
        };
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

#[allow(dead_code)]
fn read_image(img_path: &str) {
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
            let mut pixel: Rgba<u8> = img.get_pixel(x, y); // .get_pixel_mut(x, y);
            pixel = image::Rgba([pixel[0], pixel[1], pixel[2], 0]);
            img.put_pixel(x, y, pixel);
        }
    }

    img.save(format!("copia_{}", img_path)).unwrap();
}

#[allow(dead_code)]
fn print_image_pixels(img_path: &str) -> io::Result<()> {
    println!("pixeles de imagen : {}", img_path);

    let img: image::DynamicImage = match image::open(img_path) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    };

    for y in 0..1 {
        // img.height() {
        for x in 0..50 {
            // img.width() {
            let pixel = img.get_pixel(x, y);
            println!("{:?}", pixel);
        }
    }
    Ok(())
}
