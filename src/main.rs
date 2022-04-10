use clap::Parser;
use std::io;

mod bin_util;
mod steg;

/// Steganography
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the image
    img_path: String,

    /// File to be stored in the image
    #[clap(short, long)]
    input_file: Option<String>,

    /// Number of bits to use when storing/reading to/from the image
    #[clap(short = 'b', long, default_value_t = 1)]
    num_bits: u8,

    /// Output filename
    #[clap(short, long, default_value_t = String::from("out"))]
    output_file: String,
}
// esteganografia
fn main() -> io::Result<()> {
    let args = Args::parse();

    match args.input_file {
        Some(input_file) => {
            store_image(
                &args.img_path,
                &input_file,
                &args.output_file,
                args.num_bits,
            );
        }
        None => {
            load_image(&args.img_path, &args.output_file, args.num_bits);
        }
    }

    Ok(())
}

fn store_image(img_path: &str, input_file: &str, output_file: &str, num_bits: u8) {
    let _nom_img = match steg::store(img_path, input_file, output_file, num_bits) {
        Ok(a) => a,
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    };
}

fn load_image(img_path: &str, output_file: &str, num_bits: u8) {
    let _a = steg::load(img_path, output_file, num_bits);
}
