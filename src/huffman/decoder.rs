use std::io::Write;
use std::io;
use std::io::{Error, ErrorKind};
use std::io::ErrorKind::InvalidData;
use crate::huffman::{update_totals, ContextItem, ContextNode, ContextTable, RO};
use crate::huffman::utils::{create_tree, update_context, update_tables};
use crate::huffman::writer::Writer;
use crate::util::binary_tree::BinaryTree;

pub fn ppm_impl(input: &Vec<u8>, contexts_number: usize, out: &mut dyn Write) -> io::Result<()> {
    let k = contexts_number + 1;
    let mut context = String::new();
    let mut context_tables = ContextTable::new();
    // k= -1
    let mut unseen_chars: Vec<&str> = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
        "r", "s", "t", "u", "v", "w", "x", "y", "z",
    ];
    let temp = input.iter().map(|c| format!("{c:08b}")).collect::<Vec<String>>();
    let temp = temp.iter().flat_map(|frase|
        frase.chars().map(|c| u8::from_str_radix(&c.to_string(), 2).unwrap())).collect::<Vec<u8>>();

    // Inicializacao das tabelas
    for i in 0..k {
        context_tables.insert(i, ContextNode::new());
    }

    context_tables.get_mut(&0).unwrap().insert("".to_string(), ContextItem::new());

    let mut buffer: Vec<u8> = vec![];
    let mut last_decoded = RO;
    let mut counter = 0;

    for byte in temp {
        buffer.push(byte);

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

        let decoded_char = decode(
            &context,
            contexts_number,
            &mut buffer,
            &mut context_tables,
            out
        );

        let decoded_char = match decoded_char {
            Some(c) => {
                c
            },
            None => {
                if buffer.len() < 8 { continue; }
                eprintln!("Failed to decode.");
                break;
            }
        };

        update_tables(
            &context,
            contexts_number,
            &decoded_char.to_string(),
            &mut context_tables,
            &mut unseen_chars
        );

        update_context(&mut context, decoded_char, contexts_number);

    }
    Ok(())
}

fn decode(frase: &String, k: usize, input: &mut Vec<u8>, tables: &mut ContextTable, writer: &mut dyn Write) -> Option<char> {
    println!("{input:?}");

    if frase.len() < k {
        decode(frase, k-1, input, tables, writer)
    }
    else if frase.len() == k && k != 0 {
        let tk = tables.get_mut(&k).unwrap();

        // Verifica se a frase existe em k
        match tk.iter().find(|context| context.0 == frase) {
            Some(context) => {
                let tree = create_tree(context.1);
                let read = BinaryTree::decode(tree, input).unwrap();

                return if read == RO {
                    decode(&frase[1..].to_string(), k - 1, input, tables, writer)
                } else {
                    //writer.write(&[read as u8])?;
                    Some(read)
                }
            }
            None => {
                decode(&frase[1..].to_string(), k-1, input, tables, writer)
            }
        }
    }
    else if frase.len() > k {
        decode(&frase[1..].to_string(), k, input, tables, writer)
    }
    else  {
        let t0 = tables.get(&0).unwrap().get("").unwrap();

        return if t0.len() == 0 {
            if input.len() != 8 {
                return None
            }
            let result = input_to_char(input);
            println!("'{input:?}' decoded to '{}'", result);
            //writer.write(&[input]).expect("fuck you");
            Some(result)
        } else {
            let tree = create_tree(t0);
            let read = BinaryTree::decode(tree, input).unwrap();

            if read == RO {
                None
            } else {
                println!("'{:08b}' ({}) decoded to '{read}'", read as u8, read as u8);
                // writer.write(&[read as u8]).unwrap();
                Some(read)
            }
        }
    }
}

fn input_to_char(input: &Vec<u8>) -> char {
    let temp = input.iter().fold("".to_string(), |acc, x| acc + &x.to_string());
    u8::from_str_radix(&temp, 2).unwrap() as char
}