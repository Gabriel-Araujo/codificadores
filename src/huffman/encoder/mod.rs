use std::io;
use std::io::Write;
use crate::DEBUG;
use crate::util::binary_tree::BinaryTree;
use super::{ContextTable, ContextNode, ContextItem, Writer, ALPHABET, RO};
use super::utils::{update_tables, update_totals, update_context, create_tree, exclusion_table};

/// O alfabeto vai de 'a' ate 'z'.
/// No contexto k=-1 o codigo de cada letra sera seu codigo em ascii
pub fn ppm_impl(
    input: &String,
    contexts_number: usize,
    out: &mut dyn Write,
) -> io::Result<()> {
    let k = contexts_number + 1;
    let mut context_tables = ContextTable::new();
    let mut context = String::new();
    let mut writer = Writer::new(out);
    // k= -1
    let mut unseen_chars = Vec::from(ALPHABET);

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

        encoder(
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


fn encoder(frase: &String, k: usize, input: &String, tables: &ContextTable, writer: &mut Writer) -> io::Result<()> {
    if frase.len() < k {
        encoder(frase, k - 1, input, tables, writer)?;
    } else if frase.len() == k && k != 0 {
        let tk = tables.get(&k).unwrap();

        // Verifica se a frase existe em k
        match tk.iter().find(|context| context.0 == frase) {
            Some(context) => {
                if context.1.contains_key(input) {
                    let code = BinaryTree::search(
                        create_tree(context.1).get_root(),
                        input.chars().nth(0).unwrap(),
                    ).unwrap();

                    // Escreve ‘input’ na saida
                    //println!("'{input}' encoded to '{code:b}' ({code})");

                    writer.write(code, false)?;
                } else { // Input nao esta no contexto
                    let code = BinaryTree::search(create_tree(context.1).get_root(), RO).unwrap();

                    // println!("'RO' encoded to '{code:b}' ({code})");
                    writer.write(code, false)?;

                    encoder(
                        &frase[1..].to_string(),
                        k - 1,
                        input,
                        &exclusion_table(tables, context, k).unwrap(),
                        writer
                    )?;
                }
            }
            None => {
                encoder(&frase[1..].to_string(), k - 1, input, tables, writer)?;
            }
        }
    } else if frase.len() > k {
        encoder(&frase[1..].to_string(), k, input, tables, writer)?;
    } else if frase.len() == k && k == 0 {
        let t0 = tables.get(&0).unwrap().get("").unwrap();

        // So eh chamado no primeiro input
        if t0.len() == 0 {
            //println!("raw '{input}' encoded to '{:08b}' ({})", input.as_bytes()[0], input.as_bytes()[0]);
            return writer.write(input.as_bytes()[0], true);
        }
        // Verifica se input existe dentro de k = 0
        match t0.iter().find(|item| item.0 == input) {
            Some(t) => {
                let code =
                    BinaryTree::search(create_tree(t0).get_root(), input.chars().nth(0).unwrap())
                        .unwrap();

                //println!("'{input}' encoded to '{code:b}' ({code})");

                writer.write(code, false)?;
            }
            None => {
                let code = BinaryTree::search(create_tree(t0).get_root(), RO).unwrap();

                //println!("'RO' encoded to '{code:b}' ({code})");
                //println!("raw '{input}' encoded to '{:08b}' ({})", input.as_bytes()[0], input.as_bytes()[0]);

                writer.write(code, false)?;
                writer.write(input.as_bytes()[0], true)?;
            }
        }
    }
    Ok(())
}