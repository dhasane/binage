use image::{DynamicImage, GenericImage, GenericImageView};
use std::char;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str;

use crate::bin_util;

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

fn movement_iter(img_width: u32, top: u8) -> impl Iterator<Item = (u32, u32, usize, usize)> {
    let max_v = std::cmp::min(8, top + 1);
    let num = max_v * 3;
    (0..).flat_map(move |n| {
        let x: u32 = n / img_width;
        let y: u32 = n % img_width;
        (0..num).map(move |v| (x, y, (v / 3) as usize, (v % 3) as usize))
    })
}

#[allow(dead_code)]
pub fn store(
    img_path: &str,
    file_path: &str,
    output_file: &str,
    num_bits: u8,
) -> io::Result<String> {
    let mut img = DynamicImage::ImageRgba8(match image::open(img_path) {
        Ok(f) => f.into_rgba8(),
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    });

    // TODO: sacar esto como un iterador, de manera que no sea
    // absolutamente necesario leer de un archivo, sino que se pueda
    // pasar un string, por ejemplo
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let bytes = reader.bytes();

    // se pone un bit en 1 al final de cada byte en caso de que no sea
    // el ultimo, haciendo que se puedan guardar tantos bytes como sea
    // necesario, a pesar de que aumentaria el uso de memoria en la
    // imagen
    let mut bits = bytes
        .flat_map(|s| {
            let byte_value: u8 = match s {
                Ok(a) => a,
                Err(err) => {
                    eprintln!("error: {}", err);
                    std::process::exit(1);
                }
            };
            let mut ret = bin_util::byte_to_bin(byte_value as u32);
            ret.push('1');
            ret.chars().collect::<Vec<_>>()
        })
        .peekable();

    let img_width = img.width();

    let pos = movement_iter(img_width, num_bits);

    for (x, y, m, n) in pos {
        let temp = bits.next();
        let end = bits.peek().is_none();
        let bit = if end {
            '0' // ignore last and add a 0
        } else {
            match temp {
                Some(bit) => bit,
                None => '0',
            }
        };
        let mut pixel = img.get_pixel(x, y);
        pixel[n] = bin_util::modify_byte(pixel[n], m, bit);
        img.put_pixel(x, y, pixel); // maybe this could be more eficient, so as to not store it after every small change

        if end {
            break;
        }
    }

    let nombre = output_file.to_string();
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
pub fn load(img_path: &str, file_path: &str, num_bits: u8) -> io::Result<()> {
    println!("leyendo imagen {} > {}", img_path, file_path);

    let mut file: std::fs::File = File::create(file_path)?;

    let img: image::DynamicImage = match image::open(img_path) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    };

    println!(" tam imagen {}:{}", img.width(), img.height());

    let img_width: u32 = img.width();

    let pos = movement_iter(img_width, num_bits);

    let mut iterator = pos.flat_map(|(x, y, m, n)| {
        let pixel = img.get_pixel(x, y);
        let mut str: String = "".to_string();
        str.push(bin_util::get_bit(pixel[n], m));
        str.chars().collect::<Vec<char>>()
    });

    loop {
        let byte: String = (1..9)
            .map(|_n| iterator.next().unwrap())
            .collect::<String>();
        let byte_value = char::from_u32(bin_util::bin_to_byte(&byte)).unwrap();
        write!(file, "{}", byte_value).unwrap();
        let end = iterator.next().unwrap();

        if end == '0' {
            break;
        }
    }

    Ok(())
}
