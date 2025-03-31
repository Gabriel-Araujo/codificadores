use std::{
    collections::BTreeMap,
    io::{self, Error, ErrorKind, Write},
};

use super::util::{binary_tree::BinaryTree, buffer::construct};
use huffman_token::Token;
use utils::{create_tree, exclusion_table, update_context, update_tables, update_totals};
use writer::Writer;

mod decoder;
mod encoder;
mod huffman_token;
mod utils;
mod writer;

//                          letra | uso e probabilidade
type ContextItem = BTreeMap<String, Token>;
//                         contexto | letra, uso e probabilidade
type ContextNode = BTreeMap<String, ContextItem>;
//                          K | contexto | letra, uso e probabilidade
type ContextTable = BTreeMap<usize, ContextNode>;

// Como so aparece de a-z, vou usar 0 como ro.
const RO: char = '~';

const ALPHABET: [&str; 26] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z",
];

pub enum Mode {
    SemiAdaptative,
    Adapdative,
}

// Determina o tamanho máximo dos contextos.
const K: usize = 5;
pub struct Huffman {}

impl Huffman {
    pub fn encode(mode: Mode, input: &String, out: &mut dyn Write) -> io::Result<()> {
        match mode {
            Mode::SemiAdaptative => Huffman::semi_encode_impl(input, out)?,
            Mode::Adapdative => encoder::ppm_impl(input, K, out)?,
        }

        // out.write("\n".as_bytes())?;
        Ok(())
    }

    pub fn decode(mode: Mode, input: &mut Vec<u8>, out: &mut dyn Write) -> io::Result<()> {
        match mode {
            Mode::SemiAdaptative => Err(Error::new(ErrorKind::Unsupported, "Not implemented")),
            Mode::Adapdative => decoder::ppm_impl(input, K, out),
        }
    }
}

// Encoders
impl Huffman {
    fn semi_encode_impl(input: &String, out: &mut dyn Write) -> io::Result<()> {
        let table = construct(input);
        let tree = BinaryTree::new(table);
        let mut writer = Writer::new(out);

        input.chars().for_each(|c| {
            let code = BinaryTree::search(tree.get_root(), c).expect("failed to encode.");
            print!("{c} = {code:b} - ");

            writer.write(code, false).unwrap();
        });

        writer.flush()?;

        Ok(())
    }
}
