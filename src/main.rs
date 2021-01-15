#[allow(unused_imports)]
use std::io::{self, prelude::*, BufReader};

#[allow(unused_imports)]
use std::str;

#[allow(unused_imports)]
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Pixel, Rgb, Rgba};

#[allow(unused_imports)]
use std::env;

use std::fs::File;

// use crate::color::{self, IntoColor};

// esteganografia
fn main() -> io::Result<()> {
    // TODO: descomentar esto
    let args: Vec<String> = env::args().collect();
    let nom_img = match store_f_in_image(&args[1], &args[2]) {
        Ok(a) => a,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    };

    // let mut byte = 10;
    // println!("byte original  0b{:08b}", byte);
    // byte = modify_byte(byte, 0, '0');
    // println!("byte resultado 0b{:08b}", byte);

    // println!("{}", nom_img);

    read_f_in_image(&nom_img, "esto_es_prueba.txt");

    // print_image_pixels(&nom_img)

    // let aa : Vec<u32> = (0..50_u32)
    //     .map (|x| x + 1)
    //     .collect()
    //     ;
    // println!("{:?}", aa);

    Ok(())
}

fn modify_byte(mut byte: u8, pos: usize, bit: char) -> u8 {
    let byte_mask: u8 = 2_u8.pow(pos as u32);
    let change = get_bit(byte, pos);
    // println!("{:b}", byte_mask);
    if change != bit {
        byte ^= byte_mask; // Toggle bit
    }
    byte
}

fn get_bit(byte: u8, pos: usize) -> char {
    format!("{:b}", byte).chars().rev().nth(pos).unwrap()
}

fn store_f_in_image(img_path: &str, file_path: &str) -> io::Result<String> {
    let mut img = DynamicImage::ImageRgba8(match image::open(img_path) {
        Ok(f) => f.into_rgba8(),
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    });

    // posiciones en x y y de la imagen
    let mut pos_x: u32 = 0;
    let mut pos_y: u32 = 0;
    // posicion dentro del pixel -> {0..3}
    let mut pixel_pos = 0;

    // cantidad de bits para ificar en el byte
    // let pixel_internal_max = 2;
    // posicion que se esta modificando del byte
    // TODO: pensar en una forma para que se puedan recorrer varios bits de un byte
    // probablemente sea otro ciclo, pero me gustaria pensar en una forma mas elegante
    let pixel_internal_pos = 0;

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for byte in reader.bytes() {
        let val = byte?;
        println!("{:b} -> {}", val, val);
        // para cada bit dentro del byte
        for bit in format!("{:b}", val).chars() {
            print!("{} -> ", bit);
            println!("{}:{}:{}", pos_x, pos_y, pixel_pos);
            // conseguir pixel
            let mut pixel = img.get_pixel(pos_x, pos_y);
            print!("{:?} -> ", pixel);

            // modificar pixel
            print!("({:?} | ", pixel[pixel_pos]);
            pixel[pixel_pos] = modify_byte(pixel[pixel_pos], pixel_internal_pos, bit);
            print!("{:?}) -> ", pixel[pixel_pos]);

            // guardar pixel
            img.put_pixel(
                pos_x, pos_y, // Rgba<u8>::from([pixel[0], pixel[1],pixel[2], byte])
                pixel,
            );

            // mover
            pixel_pos += 1;
            if pixel_pos > 3 {
                pixel_pos = 0;
                if pos_x > img.width() {
                    pos_x = 0;
                    pos_y += 1;
                } else {
                    pos_x += 1;
                }
            }

            println!("{:?}", pixel);
        }
    }

    let nombre = format!("copia_esteg_{}", img_path);
    let res = img.save(&nombre);
    Ok(match res {
        Ok(()) => nombre,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    })
}

// pub fn decode_binary(s: &str) -> Result<Vec<u8>, ParseIntError> {
//     (0..s.len())
//         .step_by(9)
//         .map(|i| u8::from_str_radix(&s[i..i + 8], 2))
//         .collect()
// }

#[allow(dead_code)]
fn read_f_in_image(img_path: &str, file_path: &str) -> io::Result<()> {
    println!("leyendo {}", img_path);
    println!("creando {}", file_path);

    let mut file: std::fs::File = File::create(file_path)?;

    let img: image::DynamicImage = match image::open(img_path) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    };

    println!(" tam imagen {}:{}", img.width(), img.height());

    for y in 0..img.height() {
        let mut cad: String = "".to_string();

        for a in (0..img.width()).map(|x| {
            // println!("{}:{}  -> {:?}", x, y, img.get_pixel(x, y));
            let mut str: String = "".to_string();
            str.push(get_bit(img.get_pixel(x, y)[0], 0));
            str.push(get_bit(img.get_pixel(x, y)[1], 0));
            str.push(get_bit(img.get_pixel(x, y)[3], 0));
            str
        }) {
            // ir sumando los bits en una cadena y al haber por lo menos  8,
            // se convierten a caracter y se quitan los caracteres usados
            cad.push_str(&a);
            if cad.chars().count() >= 8 {
                let car: char = u8::from_str_radix(&cad[0..8], 2).unwrap() as char;
                // println!("{} -> {}", cad[0..8].to_string(), car);
                write!(file, "{}", car).unwrap();
                cad = cad[8..].to_string();
            }
        }

        // println!("{:?}", aa);

        // let pal = str::from_utf8(&aa).unwrap();
        // println!("{}", pal);

        // // println!("{:?}", aa);

        // match file.write(&aa) {
        //     Ok(f) => f,
        //     Err(err) => {
        //         eprintln!("error: {}", err);
        //         std::process::exit(1);
        //     }
        // };
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
