use std::{
    collections::BTreeMap,
    io::{self, Write, Error, ErrorKind},
};

use huffman_token::Token;
use writer::Writer;
use utils::{update_context, update_tables, update_totals, create_tree, exclusion_table};
use super::util::{binary_tree::BinaryTree, buffer::construct};

mod huffman_token;
mod writer;
mod encoder;
mod decoder;
mod utils;

//                          letra | uso e probabilidade
type ContextItem = BTreeMap<String, Token>;
//                         contexto | letra, uso e probabilidade
type ContextNode = BTreeMap<String, ContextItem>;
//                          K | contexto | letra, uso e probabilidade
type ContextTable = BTreeMap<usize, ContextNode>;

// Como so aparece de a-z, vou usar 0 como ro.
const RO: char = '0';

pub enum Mode {
    SemiAdaptative,
    Adapdative,
}

const K: usize = 2;
pub struct Huffman {}

impl Huffman {
    pub fn encode(mode: Mode, input: &String, out: &mut dyn Write) -> io::Result<()> {
        match mode {
            Mode::SemiAdaptative => Huffman::semi_encode_impl(input, out)?,
            Mode::Adapdative => Huffman::ppm_encode_impl(input, K, out)?,
        }

        // out.write("\n".as_bytes())?;
        Ok(())
    }

    pub fn decode(mode: Mode, input: &Vec<u8>, out: &mut dyn Write) -> io::Result<()> {
        match mode {
            Mode::SemiAdaptative => {Err(Error::new(ErrorKind::Unsupported, "Not implemented"))}
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
    /// O alfabeto vai de 'a' ate 'z'.
    /// No contexto k=-1 o codigo de cada letra sera seu codigo em ascii
    fn ppm_encode_impl(
        input: &String,
        contexts_number: usize,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        let k = contexts_number + 1;
        let mut context_tables = ContextTable::new();
        let mut context = String::new();
        let mut writer = Writer::new(out);
        // k= -1
        let mut unseen_chars: Vec<&str> = vec![
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z",
        ];

        for i in 0..k {
            context_tables.insert(i, ContextNode::new());
        }
        context_tables
            .get_mut(&0)
            .unwrap()
            .insert("".to_string(), ContextItem::new());

        for c in input.chars() {
            // Remove RO de K=0 se K=-1 ficar vazia.
            if unseen_chars.len() == 0
                && context_tables
                    .get(&0)
                    .unwrap()
                    .get("")
                    .unwrap()
                    .contains_key(&RO.to_string())
            {
                let t0 = context_tables.get_mut(&0).unwrap().get_mut("").unwrap();

                t0.remove(&RO.to_string());
                update_totals(t0);
            }

            codify(
                &context,
                contexts_number,
                &c.to_string(),
                &mut context_tables,
                &mut writer
            )?;
            update_tables(
                &context,
                contexts_number,
                &c.to_string(),
                &mut context_tables,
                &mut unseen_chars,
            );
            update_context(&mut context, c, contexts_number); // Nao pode ser K devido a como Rust lida com slices
        }

        // context_tables.iter().for_each(|f| println!("{f:?}"));
        writer.flush()?;
        Ok(())
    }

    fn ppm_decode_impl(input: &String, k: usize, out: &mut dyn Write) -> io::Result<()> {
        let contexts_number = k + 1;
        let mut context_tables = ContextTable::new();
        let mut context = String::new();
        let mut reader = BitReader::new(input.as_bytes());
        let mut unseen_chars: Vec<&str> = vec![
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z",
        ];

        // Inicializa as tabelas de contexto
        for i in 0..=contexts_number {
            context_tables.insert(i, ContextNode::new());
        }
        context_tables
            .get_mut(&0)
            .unwrap()
            .insert("".to_string(), ContextItem::new());

        loop {
            // Remove RO de K=0 se K=-1 ficar vazia
            if unseen_chars.len() == 0
                && context_tables
                .get(&0)
                .unwrap()
                .get("")
                .unwrap()
                .contains_key(&RO.to_string())
            {
                let t0 = context_tables.get_mut(&0).unwrap().get_mut("").unwrap();
                t0.remove(&RO.to_string());
                update_totals(t0);
            }

            // Tenta decodificar o próximo caractere
            let decoded_char = match decode_next_char(&context, contexts_number, &context_tables, &mut reader) {
                Ok(Some(c)) => c,
                Ok(None) => break, // Fim da entrada
                Err(e) => return Err(e),
            };

            // Escreve o caractere decodificado na saída
            //out.write_all(&[decoded_char as u8])?;
            println!("{decoded_char}");
            // Atualiza as tabelas de contexto com o novo caractere
            update_tables(
                &context,
                contexts_number,
                &decoded_char.to_string(),
                &mut context_tables,
                &mut unseen_chars,
            );

            // Atualiza o contexto
            update_context(&mut context, decoded_char, contexts_number);
        }

        Ok(())
    }
}

fn decode_next_char(
    context: &String,
    k: usize,
    tables: &ContextTable,
    reader: &mut BitReader,
) -> io::Result<Option<char>> {
    if context.len() < k {
        return decode_next_char(context, k - 1, tables, reader);
    } else if context.len() == k && k != 0 {
        let tk = tables.get(&k).unwrap();

        match tk.iter().find(|ctx| ctx.0 == context) {
            Some(ctx) => {
                let tree = create_tree(ctx.1);
                match BinaryTree::decode_v0(tree.get_root(), reader) {
                    Ok(Some(RO)) => {
                        // Encontramos um escape, tentar no contexto inferior
                        let next_context = context[1..].to_string();
                        decode_next_char(&next_context, k - 1, tables, reader)
                    }
                    Ok(Some(c)) => Ok(Some(c)),
                    Ok(None) => Ok(None),
                    Err(e) => Err(e),
                }
            }
            None => {
                // Contexto não encontrado, tentar no nível inferior
                let next_context = context[1..].to_string();
                decode_next_char(&next_context, k - 1, tables, reader)
            }
        }
    } else if context.len() > k {
        let next_context = context[1..].to_string();
        decode_next_char(&next_context, k, tables, reader)
    } else if context.len() == k && k == 0 {
        let t0 = tables.get(&0).unwrap().get("").unwrap();

        if t0.is_empty() {
            // Caso especial - ler byte diretamente (k=-1)
            let byte = reader.read_byte()?;
            return Ok(Some(byte as char));
        }

        let tree = create_tree(t0);
        match BinaryTree::decode_v0(tree.get_root(), reader) {
            Ok(Some(RO)) => {
                // Escape - ler byte diretamente
                let byte = reader.read_byte()?;
                Ok(Some(byte as char))
            }
            Ok(Some(c)) => Ok(Some(c)),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    } else {
        Ok(None)
    }
}

// Implementação auxiliar do BitReader para ler bits da entrada
pub struct BitReader<'a> {
    bytes: &'a [u8],
    current_byte: u8,
    bit_pos: u8,
    byte_pos: usize,
}

impl<'a> BitReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        BitReader {
            bytes,
            current_byte: 0,
            bit_pos: 8, // Começa sem byte atual
            byte_pos: 0,
        }
    }

    pub fn read_bit(&mut self) -> io::Result<u8> {
        if self.bit_pos >= 8 {
            if self.byte_pos >= self.bytes.len() {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No more bits"));
            }
            self.current_byte = self.bytes[self.byte_pos];
            self.byte_pos += 1;
            self.bit_pos = 0;
        }

        let bit = (self.current_byte >> (7 - self.bit_pos)) & 1;
        self.bit_pos += 1;
        Ok(bit)
    }

    fn read_byte(&mut self) -> io::Result<u8> {
        let mut byte = 0;
        for _ in 0..8 {
            let bit = self.read_bit()?;
            byte = (byte << 1) | bit;
        }
        Ok(byte)
    }
}

fn codify(frase: &String, k: usize, input: &String, tables: &ContextTable, writer: &mut Writer) -> io::Result<()> {
    if frase.len() < k {
        codify(frase, k - 1, input, tables, writer)?;
    } else if frase.len() == k && k != 0 {
        let tk = tables.get(&k).unwrap();

        // Verifica se a frase existe em k
        match tk.iter().find(|context| context.0 == frase) {
            Some(context) => {
                if context.1.contains_key(input) {
                    let code = BinaryTree::search(
                        create_tree(context.1).get_root(),
                        input.chars().nth(0).unwrap(),
                    )
                    .unwrap();
                    // Escreve ‘input’ na saida
                    println!("'{input}' encoded to '{code:b}' ({code})");
                    writer.write(code, false)?;
                } else {
                    let code = BinaryTree::search(create_tree(context.1).get_root(), RO).unwrap();
                    println!("'RO' encoded to '{code:b}' ({code})");
                    writer.write(code, false)?;
                    codify(
                        &frase[1..].to_string(),
                        k - 1,
                        input,
                        &exclusion_table(tables, context, k).unwrap(),
                        writer
                    )?;
                }
            }
            None => {
                codify(&frase[1..].to_string(), k - 1, input, tables, writer)?;
            }
        }
    } else if frase.len() > k {
        codify(&frase[1..].to_string(), k, input, tables, writer)?;
    } else if frase.len() == k && k == 0 {
        let t0 = tables.get(&0).unwrap().get("").unwrap();

        if t0.len() == 0 {
            println!("raw '{input}' encoded to '{:08b}' ({})", input.as_bytes()[0], input.as_bytes()[0]);
            return writer.write(input.as_bytes()[0], true);
        }
        // Verifica se input existe dentro de k = 0
        match t0.iter().find(|item| item.0 == input) {
            Some(t) => {
                let code =
                    BinaryTree::search(create_tree(t0).get_root(), input.chars().nth(0).unwrap())
                        .unwrap();
                println!("'{input}' encoded to '{code:b}' ({code})");
                writer.write(code, false)?;
            }
            None => {
                let code = BinaryTree::search(create_tree(t0).get_root(), RO).unwrap();
                println!("'RO' encoded to '{code:b}' ({code})");
                println!("raw '{input}' encoded to '{:08b}' ({})", input.as_bytes()[0], input.as_bytes()[0]);
                writer.write(code, false)?;
                writer.write(input.as_bytes()[0], true)?;
            }
        }
    }
    Ok(())
}