use crate::huffman::decoder::decode;
use crate::huffman::utils::{create_tree, exclusion_table};
use crate::huffman::{ContextTable, RO};
use crate::util::binary_tree::BinaryTree;
use std::collections::VecDeque;

pub fn input_to_char(input: &Vec<u8>) -> char {
    let temp = input
        .iter()
        .fold("".to_string(), |acc, x| acc + &x.to_string());
    u8::from_str_radix(&temp, 2).unwrap() as char
}

pub fn decode_raw(input: &Vec<u8>) -> char {
    let mut input_d = VecDeque::from(input.clone());
    let mut temp: Vec<u8> = vec![];

    for _ in 0..8 {
        temp.push(input_d.pop_front().unwrap());
    }

    input_to_char(&temp)
}

pub fn decode_inside(input: &mut Vec<u8>) -> char {
    let mut temp: Vec<u8> = vec![];

    for _ in 0..8 {
        temp.push(input.pop().unwrap());
    }
    input_to_char(&temp)
}

pub fn teste(input: &mut Vec<u8>) -> u8 {
    let mut temp: Vec<u8> = vec![];

    for _ in 0..8 {
        temp.push(input.pop().unwrap());
    }

    temp.reverse();

    input_to_char(&temp) as u8
}

pub fn decode_exclusion(
    frase: &String,
    k: usize,
    input: &Vec<u8>,
    tables: &ContextTable,
    bit_buffer: &mut Vec<u8>,
) -> Option<char> {
    let mut _input = input.clone();
    _input.push(bit_buffer.pop().unwrap());

    if frase.len() < k {
        decode_exclusion(frase, k - 1, &_input, tables, bit_buffer)
    } else if frase.len() > k {
        decode_exclusion(&frase[1..].to_string(), k, &_input, tables, bit_buffer)
    } else if frase.len() == k && k != 0 {
        let tk = tables.get(&k).unwrap();

        match tk.iter().find(|context| context.0 == frase) {
            Some(context) => {
                let tree = create_tree(context.1);
                let decoded = BinaryTree::decode(tree, &_input).unwrap();
                if decoded != RO {
                    Some(decoded)
                }
                // Se eu consigo encontrar um caracter valido retorno ele
                else {
                    decode_exclusion(
                        &frase[1..].to_string(),
                        k - 1,
                        &vec![],
                        &exclusion_table(tables, context, k).unwrap(),
                        bit_buffer,
                    )
                }
            }
            None => decode_exclusion(&frase[1..].to_string(), k - 1, &_input, tables, bit_buffer),
        }
    } else if frase.len() == k && k == 0 {
        let t0 = tables.get(&0).unwrap().get("").unwrap();

        if t0.is_empty() {
            panic!("Erro na decode_exclusion.");
        }

        let tree = create_tree(t0);
        match BinaryTree::decode(tree, &_input) {
            Some(t) => {
                return if t == RO {
                    Some(decode_inside(bit_buffer))
                } else {
                    Some(t)
                };
            }
            None => decode_exclusion(frase, k, &_input, tables, bit_buffer),
        }
    } else {
        None
    }
}
