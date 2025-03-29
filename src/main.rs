use std::{
    fs::File,
    io::{self, Read},
    time::Instant,
};

use huffman::{
    Huffman,
    Mode::{self},
};
use util::io::{get_file_content, get_other_vars};

mod huffman;
mod shannon_fano;
mod util;

fn main() -> io::Result<()> {
    let args = get_other_vars();
    let save = args.iter().any(|i| i == "--save");
    let encode = args.iter().any(|i| i == "--encode");
    let decode = args.iter().any(|i| i == "--decode");
    let _static = args.iter().any(|i| i == "--static");
    // let input = get_file_content();

    let input = "abracadabra".to_string();
    /*
        let buffer = match _static {
            true => get_default_weights(),
            false => construct(&input),
        };

         Using Shannon Fano encoding
        let buffer = generate_table(buffer, "".to_string());
        if show_tables {
            buffer.iter().for_each(|t| println!("{:?}", t));
        }
    */
    let mut a = File::create("decoded_out.txt")?;
    Huffman::encode(Mode::Adapdative, &input, &mut a)?;

    if encode {
        let now = Instant::now();
        if save {
            let mut a = File::create("decoded_out.txt")?;
            Huffman::encode(Mode::Adapdative, &input, &mut a)?;
        } else {
            Huffman::encode(Mode::Adapdative, &input, &mut io::stdout())?;
        }
        println!("took {:?} to encode.", now.elapsed());
    }
    if decode {
        let mut buf = vec![];
        let mut a = File::open("decoded_out.txt")?;
        a.read_to_end(&mut buf)?;

        buf.iter().for_each(|c| println!("{c:b}"));
    }

    println!("Reading from buffer");

    let mut buf = Default::default();
    let mut a = File::open("decoded_out.txt")?;
    a.read_to_end(&mut buf)?;

    // println!("{:?}", args);

    Huffman::decode(Mode::Adapdative, &buf, &mut a)?;
    Ok(())
}
