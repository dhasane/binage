/// bit goes into byte at it's position pos
pub fn modify_byte(mut byte: u8, pos: usize, bit: char) -> u8 {
    let base: u8 = 2;
    let byte_mask: u8 = base.pow(pos as u32);
    let change = get_bit(byte, pos);
    if change != bit {
        byte ^= byte_mask; // Toggle bit
    }
    byte
}

pub fn get_bit(byte: u8, pos: usize) -> char {
    format!("{:08b}", byte).chars().rev().nth(pos).unwrap()
}

// convierte de u32 a una cadena de bits
pub fn byte_to_bin(cad: u32) -> String {
    // quitar los primeros 2 caracteres, que son 0b
    let ret = format!("{:010b}", cad)[2..].to_string();
    ret
}

/// convierte una cadena de bits a u32
pub fn bin_to_byte(cad: &str) -> u32 {
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
