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
    // let nom_img = match store_f_in_image(&args[1], &args[2]) {
    //     Ok(a) => a,
    //     Err(err) => {
    //         eprintln!("error: {}", err);
    //         std::process::exit(1);
    //     }
    // };

    // let mut byte = 10;
    // println!("byte original  0b{:08b}", byte);
    // byte = modify_byte(byte, 0, '0');
    // println!("byte resultado 0b{:08b}", byte);

    // println!("{}", nom_img);

    // let _a = hmmmmmmm(&nom_img, "esto_es_prueba.txt");
    let _nom_img = match store(&args[1], &args[2]) {
        Ok(a) => a,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    };
    // let _a = ham(&nom_img, "pruebaaaa.txt");
    // let _a = read_f_in_image(&nom_img, "pruebaaaa.txt");

    // print_image_pixels(&nom_img)

    // let aa : Vec<u32> = (0..50_u32)
    //     .map (|x| x + 1)
    //     .collect()
    //     ;
    // println!("{:?}", aa);

    Ok(())
}

fn modify_byte(mut byte: u8, pos: u8, bit: char) -> u8 {
    let base: u8 = 2;
    let byte_mask: u8 = base.pow(pos as u32);
    let change = get_bit(byte, pos as usize);
    // println!("{:b}", byte_mask);
    if change != bit {
        byte ^= byte_mask; // Toggle bit
    }
    byte
}

fn get_bit(byte: u8, pos: usize) -> char {
    format!("{:b}", byte).chars().rev().nth(pos).unwrap()
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

    // TODO: leer esto de alguna parte
    let length = 30;

    let max_width: u64 = img.width() as u64;

    for position in (0..length).step_by(3) {
        let x = (position / max_width) as u32;
        let y = (position % max_width) as u32;
        let pixel = img.get_pixel(x, y);
        for channel in 0..3_usize {
            println!("{}", byte_to_bin(pixel[channel] as usize));
        }
    }
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
                let car = u8::from_str_radix(&cad[0..8], 2).unwrap();
                println!("{} -> {}", cad[0..8].to_string(), car);
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

fn byte_to_bin(cad: usize) -> String {
    // let ret = format!("{:#08}", cad);
    let ret = format!("{:010b}", cad)[2..].to_string();
    // println!("{} -> {} ({})", cad, ret, bin_to_byte(&ret));
    ret
}

fn bin_to_byte(cad: &str) -> usize {
    let mut val: usize = 0;
    let mut pos: u32 = 0;
    for ch in cad.chars().rev() {
        if ch == '1' {
            val += 2_usize.pow(pos);
        }
        pos += 1;
    }
    val
}

#[allow(dead_code)]
fn store(img_path: &str, file_path: &str) -> io::Result<String> {
    let mut img = DynamicImage::ImageRgba8(match image::open(img_path) {
        Ok(f) => f.into_rgba8(),
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    });

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let bytes = reader.bytes();

    let metadata = std::fs::metadata(file_path)?;
    let meta_length: usize = metadata.len() as usize;

    // se pone un bit en 1 al final de cada byte en caso de que no sea
    // el ultimo, haciendo que se puedan guardar tantos bytes como sea
    // necesario, a pesar de que aumentaria el uso de memoria en la
    // imagen
    let file_bits: Vec<char> = bytes
        .flat_map(|s| {
            let byte_value: u8 = match s {
                Ok(a) => a,
                Err(err) => {
                    eprintln!("error: {}", err);
                    std::process::exit(1);
                }
            };
            let mut ret = byte_to_bin(byte_value as usize).chars().collect::<Vec<_>>();
            ret.extend(vec!['1']);
            ret
        })
        .collect();

    // agregar la longitud del archivo
    let mut bits: Vec<char> = byte_to_bin(meta_length).chars().collect::<Vec<char>>();

    // agregar separador // se va a dejar como que los primeros 8 son
    // el tam y el resto ya es informacion, que hace que el archivo no
    // sea tan grande, pero de momento funciona bien para probar el
    // resto
    // bits.extend(byte_to_bin(':' as usize).chars().collect::<Vec<char>>());

    // agregar los bits del archivo
    bits.extend(file_bits);

    let length: usize = bits.len();
    let _got = std::mem::replace(&mut bits[length - 1], '0');

    let max_width: usize = img.width() as usize;

    println!("{:?}", bits);

    println!("inicio---");
    for position in (0..length).step_by(3) {
        let x = (position / max_width) as u32;
        let y = (position % max_width) as u32;

        let mut pixel = img.get_pixel(x, y);
        for channel in 0..3_usize {
            let local = position + channel;
            if local < bits.len() {
                let bit = bits[local];
                pixel[channel] = modify_byte(pixel[channel], 0, bit);
            }
        }
        img.put_pixel(x, y, pixel);
    }

    println!("tam {:?}", length);

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

#[allow(dead_code)]
fn ham(img_path: &str, file_path: &str) -> io::Result<()> {
    println!("leyendo {}", img_path);
    println!("creando {}", file_path);

    let mut _file: std::fs::File = File::create(file_path)?;

    let img: image::DynamicImage = match image::open(img_path) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    };

    println!(" tam imagen {}:{}", img.width(), img.height());

    // TODO: hacer esto de manera organizada
    let length = 30;

    let max_width: u64 = img.width() as u64;

    let cadena_completa = ((0..length).step_by(3))
        .map(|position| {
            // println!("{}:{}  -> {:?}", x, y, img.get_pixel(x, y));
            let x = (position / max_width) as u32;
            let y = (position % max_width) as u32;
            let pixel = img.get_pixel(x, y);
            let mut str: String = "".to_string();
            for channel in 0..3_usize {
                println!("{}", byte_to_bin(pixel[channel] as usize));
                str.push(get_bit(img.get_pixel(x, y)[0], 0));
            }
            str
        })
        .collect::<String>();

    // let c_vec: Vec<String> =
    println!("{}", cadena_completa);
    // cadena_completa.chars().step_by(8).map(|c| {
    //     println!("{}", c);
    // });

    // let resultado = match decode_binary(&cadena_completa) {
    //     Ok(a) => a,
    //     Err(err) => {
    //         eprintln!("error: {}", err);
    //         std::process::exit(1);
    //     }
    // };

    // TODO: esto esta fallando

    println!("cadenas:");
    for cad in (0..cadena_completa.len()).step_by(8) {
        let tentative = cad + 8;
        let top = if tentative > cadena_completa.len() {
            cadena_completa.len()
        } else {
            tentative
        };
        let subcadena = &cadena_completa[cad..top];
        println!("{} -> ", subcadena);
    }

    // for res in resultado {
    //     println!("{}", res);
    // }

    Ok(())
}
