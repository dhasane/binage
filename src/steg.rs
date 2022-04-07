use image::{DynamicImage, GenericImage, GenericImageView};
use std::char;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str;

/// bit goes into byte at it's position pos
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

// convierte de u32 a una cadena de bits
fn byte_to_bin(cad: u32) -> String {
    // quitar los primeros 2 caracteres, que son 0b
    let ret = format!("{:010b}", cad)[2..].to_string();
    ret
}

// convierte una cadena de bits a u32
fn bin_to_byte(cad: &str) -> u32 {
    let mut val: u32 = 0;
    let mut pos: u32 = 0;
    for ch in cad.chars().rev() {
        if ch == '1' {
            val += 2_u32.pow(pos);
        }
        pos += 1;
    }
    val
}

#[allow(dead_code)]
pub fn store(img_path: &str, file_path: &str, output_file: &str) -> io::Result<String> {
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
            let mut ret = byte_to_bin(byte_value as u32);
            ret.push('1');
            // println!("{:?} -> {}", ret, byte_value);
            ret.chars().collect::<Vec<_>>()
        })
        .peekable();

    let img_width = img.width();

    let pos = (0..)
        .flat_map(|n| {
            let x: u32 = n / img_width;
            let y: u32 = n % img_width;
            vec![(x, y, 1), (x, y, 2), (x, y, 3)]
        })
        .into_iter();

    for (x, y, n) in pos {
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
        // println!("{:?}, end:{}", temp, end);
        // println!("{},{},{} => {}", x, y, n, bit);
        let mut pixel = img.get_pixel(x, y);
        pixel[n] = modify_byte(pixel[n], 0, bit);
        img.put_pixel(x, y, pixel); // maybe this could be more eficient, so as to not store it after every small change

        if end {
            println!("saliendo");
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
pub fn load(img_path: &str, file_path: &str) -> io::Result<()> {
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

    let img_width: u32 = img.width();

    let pos = (0..)
        .flat_map(|n| {
            let x: u32 = n / img_width;
            let y: u32 = n % img_width;
            vec![(x, y, 1), (x, y, 2), (x, y, 3)]
        })
        .into_iter();

    let mut iterator = pos.flat_map(|(x, y, n)| {
        let pixel = img.get_pixel(x, y);
        let mut str: String = "".to_string();
        str.push(get_bit(pixel[n], 0));
        str.chars().collect::<Vec<char>>()
    });

    loop {
        let byte: String = (1..9)
            .map(|_n| iterator.next().unwrap())
            .collect::<String>();
        let byte_value = char::from_u32(bin_to_byte(&byte)).unwrap();
        write!(file, "{}", byte_value).unwrap();
        let end = iterator.next().unwrap();

        if end == '0' {
            break;
        }
    }

    Ok(())
}
