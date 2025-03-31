use std::collections::VecDeque;
use std::io;
use std::io::{BufRead, Read, Write};

use crate::util::binary_tree::BinaryTree;

use super::utils::{create_tree, exclusion_table, update_context, update_tables};
use super::{ALPHABET, ContextItem, ContextNode, ContextTable, RO};
use util::*;

mod util;

pub fn ppm_impl(
    input: &mut Vec<u8>,
    contexts_number: usize,
    out: &mut dyn Write,
) -> io::Result<()> {
    let k = contexts_number + 1;
    let mut context_tables = ContextTable::new();
    let mut context = String::new();
    // k= -1
    let mut unseen_chars: Vec<&str> = Vec::from(ALPHABET);

    // Inicializacao das tabelas
    for i in 0..k {
        context_tables.insert(i, ContextNode::new());
    }

    context_tables
        .get_mut(&0)
        .unwrap()
        .insert("".to_string(), ContextItem::new());

    // Transformacao do vetor de bytes em um vetor de bits
    let bit_array = input
        .iter()
        .map(|c| format!("{c:08b}"))
        .collect::<Vec<String>>();
    let mut bit_array = bit_array
        .iter()
        .flat_map(|frase| {
            frase
                .chars()
                .map(|c| u8::from_str_radix(&c.to_string(), 2).unwrap())
        })
        .collect::<Vec<u8>>();

    let a = teste(&mut bit_array);

    for _ in 0..a {
        bit_array.pop();
    }

    bit_array.reverse(); // inverte ordem por questoes de desempenho.

    let mut buffer: Vec<u8> = Vec::new();

    loop {
        if bit_array.is_empty() {
            break;
        }

        buffer.push(bit_array.pop().unwrap());

        let result = decode(
            &context,
            contexts_number,
            &buffer,
            &mut context_tables,
            &mut bit_array,
        );

        let decoded = match result {
            Some(c) => {
                buffer = vec![];

                c
            }
            None => {
                continue;
            }
        };

        if decoded != RO {
            update_tables(
                &context,
                contexts_number,
                &decoded.to_string(),
                &mut context_tables,
                &mut unseen_chars,
            );
            update_context(&mut context, decoded, contexts_number);
            println!("{decoded}")
        }
    }

    Ok(())
}

fn decode(
    frase: &String,
    k: usize,
    input: &Vec<u8>,
    tables: &ContextTable,
    bit_buffer: &mut Vec<u8>,
) -> Option<char> {
    if frase.len() < k {
        decode(frase, k - 1, input, tables, bit_buffer)
    } else if frase.len() == k && k != 0 {
        let tk = tables.get(&k).unwrap();

        // Verifico se a frase existe em k
        match tk.iter().find(|context| context.0 == frase) {
            Some(context) => {
                // Se existe, crio a árvore e tento achar uma folha.
                let tree = create_tree(context.1);
                let decoded = BinaryTree::decode(tree, input)?;

                if decoded != RO {
                    Some(decoded)
                }
                // Se eu consigo encontrar um caracter valido retorno ele
                else {
                    // Se eu encontro RO aplico recursivamente a tabela de exclusao

                    decode_exclusion(
                        &frase[1..].to_string(),
                        k - 1,
                        &vec![],
                        &exclusion_table(tables, context, k).unwrap(),
                        bit_buffer,
                    )
                }
            }
            None => {
                // Se a frase nao existe em k
                decode(&frase[1..].to_string(), k - 1, input, tables, bit_buffer)
            }
        }
    } else if frase.len() > k {
        decode(&frase[1..].to_string(), k, input, tables, bit_buffer)
    } else if frase.len() == k && k == 0 {
        let t0 = tables.get(&0).unwrap().get("").unwrap();

        // So eh executado no primero elemento a ser decodificado
        if t0.is_empty() && input.len() == 8 {
            return Some(decode_raw(input));
        } else if t0.is_empty() {
            return None;
        }

        let tree = create_tree(t0);
        match BinaryTree::decode(tree, input) {
            Some(t) => {
                return if t == RO {
                    Some(decode_inside(bit_buffer))
                } else {
                    Some(t)
                };
            }
            None => None,
        }
    } else {
        None
    }
}
