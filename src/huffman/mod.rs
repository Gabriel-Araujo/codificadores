use std::io::{self, Write};

use crate::util::{binary_tree::BinaryTree, buffer::construct};

pub enum Mode {
    SemiAdaptative,
    Adapdative,
}

pub struct Huffman {}

impl Huffman {
    pub fn encode(mode: Mode, input: &String, out: &mut dyn Write) -> io::Result<()> {
        match mode {
            Mode::SemiAdaptative => Huffman::semi_encode_impl(input, out)?,
            Mode::Adapdative => {}
        }
        out.write("\n".as_bytes())?;
        Ok(())
    }

    pub fn decode() {}
}

// Encoders
impl Huffman {
    fn semi_encode_impl(input: &String, out: &mut dyn Write) -> io::Result<()> {
        let table = construct(input);
        let tree = BinaryTree::new(table);
        let mut buffer = "".to_owned();
        let mut extra = "".to_owned();

        input.chars().for_each(|c| {
            let code = BinaryTree::search(tree.get_root(), c).expect("failed to encode.");
            print!("{c} = {code:b} - ");

            format!("{code:b}").chars().for_each(|f| {
                if buffer.len() < 8 {
                    buffer.push(f);
                } else {
                    extra.push(f);
                }
            });
            if buffer.len() == 8 {
                let bin = u8::from_str_radix(buffer.as_str(), 2).unwrap();
                println!("{buffer} was written in the buffer.");
                out.write(&[bin])
                    .expect("Error while writing on the buffer");
                buffer = extra.clone();
                extra = "".to_owned();
            }
            if !extra.is_empty() {
                println!("shit {extra:?} {}", extra.len())
            }
        });
        if !buffer.is_empty() {
            out.write(&[u8::from_str_radix(buffer.as_str(), 2).unwrap()])?;
        }

        Ok(())
    }
}
