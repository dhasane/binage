use image::{DynamicImage, GenericImage, GenericImageView};
use std::char;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str;

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
    let file_bits: Vec<char> = bytes
        .flat_map(|s| {
            let byte_value: u8 = match s {
                Ok(a) => a,
                Err(err) => {
                    eprintln!("error: {}", err);
                    std::process::exit(1);
                }
            };
            let mut ret = byte_to_bin(byte_value as u32).chars().collect::<Vec<_>>();
            ret.extend(vec!['1']);
            println!("{:?} -> {}", ret, byte_value);
            ret
        })
        .collect();

    // agregar la longitud del archivo
    let mut bits: Vec<char> = file_bits;

    let length: usize = bits.len();
    bits[length - 1] = '0';

    let max_width: usize = img.width() as usize;

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

    let max_width: u64 = img.width() as u64;

    // itera sobre cada uno de los pixeles y saca los valores de cada canal
    let iterator = ((0..).step_by(3))
        .map(|position| {
            let x = (position / max_width) as u32;
            let y = (position % max_width) as u32;
            let pixel = img.get_pixel(x, y);
            let mut str: String = "".to_string();
            for channel in 0..3_usize {
                str.push(get_bit(pixel[channel], 0));
            }
            str.chars().collect::<Vec<char>>()
        })
        .enumerate();

    let mut byte: String = "".to_string();
    for it in iterator {
        let restr: String = it.1.into_iter().collect();
        byte.push_str(&restr);
        if it.0 % 3 == 2 {
            let flag = byte.pop(); // el ultimo elemento no es parte del byte
            let byte_value = char::from_u32(bin_to_byte(&byte)).unwrap();
            println!("{:?} -> {}", byte, byte_value);
            write!(file, "{}", byte_value).unwrap();
            if flag == Some('0') {
                break;
            }
            byte = "".to_string();
        }
    }

    Ok(())
}
