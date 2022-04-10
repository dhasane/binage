use image::{DynamicImage, GenericImage, GenericImageView};
use std::char;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;
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

    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            println!("{:?}", pixel);
        }
    }
    Ok(())
}

fn movement_iter(
    img_width: u32,
    img_height: u32,
    top: u8,
) -> impl Iterator<Item = (u32, u32, usize, usize)> {
    let max_v = std::cmp::min(8, top);
    let num = max_v * 3;
    (0..).flat_map(move |n| {
        let x: u32 = n % img_width;
        let y: u32 = (n / img_width) % img_height; // this may overwrite data
        (0..num).map(move |v| (x, y, (v / 3) as usize, (v % 3) as usize))
    })
}

#[allow(dead_code)]
pub fn store(
    img_path: &str,
    input_file: &str,
    output_file: &str,
    num_bits: u8,
) -> io::Result<String> {
    let input_path = Path::new(input_file);
    let img_path = Path::new(img_path);
    let output_path =
        output_file.to_owned() + "." + img_path.extension().unwrap().to_str().unwrap();

    println!(
        "File {} > {} as {}",
        input_path.display(),
        img_path.display(),
        output_path
    );

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
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let bytes = reader.bytes();

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
            // se pone un bit en 1 al final de cada byte en caso de que no sea
            // el ultimo, haciendo que se puedan guardar tantos bytes como sea
            // necesario, a pesar de que aumentaria el uso de memoria en la
            // imagen. El ultimo se modifica para ser un 0,
            // significando el final del archivo.
            ret.push('1');
            ret.chars().collect::<Vec<_>>()
        })
        .peekable();

    let pos = movement_iter(img.width(), img.height(), num_bits);

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

    let res = img.save(&output_path);
    Ok(match res {
        Ok(()) => output_path,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    })
}

#[allow(dead_code)]
pub fn load(img_path: &str, output_file: &str, num_bits: u8) -> io::Result<()> {
    println!("reading image {} > {}", img_path, output_file);

    let mut file: std::fs::File = File::create(output_file)?;

    let img: image::DynamicImage = match image::open(img_path) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    };

    let pos = movement_iter(img.width(), img.height(), num_bits);

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
