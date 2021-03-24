// use crate::color::{self, IntoColor};
use std::env;
use std::io;

mod steg;

// esteganografia
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let _nom_img = match steg::store(&args[1], &args[2], &args[3]) {
        Ok(a) => a,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    };
    let _a = steg::load(&args[3], "pruebaaaa.txt");

    Ok(())
}
