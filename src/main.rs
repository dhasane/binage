use std::env;
use std::io;

mod bin_util;
mod steg;

// esteganografia
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let num_bits = 8;
    let _nom_img = match steg::store(&args[1], &args[2], &args[3], num_bits) {
        Ok(a) => a,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    };
    let _a = steg::load(&args[3], "pruebaaaa.txt", num_bits);

    Ok(())
}
