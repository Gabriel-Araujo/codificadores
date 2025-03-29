use crate::util::binary_tree::BinaryTree;
use super::huffman_token::Token;
use super::{ContextItem, ContextTable, RO};

pub fn update_totals(table: &mut ContextItem) {
    let mut counter = 0;

    table.iter().for_each(|item| counter += item.1.get_usage());
    table.iter_mut().for_each(|item| item.1.set_total(counter));
}

pub fn update_tables(
    frase: &String,
    k: usize,
    input: &String,
    tables: &mut ContextTable,
    unseen: &mut Vec<&str>,
) {
    if frase.len() == 0 && k == 0 {
        let t0 = tables.get_mut(&0).unwrap().get_mut("").unwrap();

        // Verifica se o input existe em k = 0
        match t0.iter_mut().find(|item| item.0 == input) {
            Some(t) => {
                // Se existe, incrementa o seu uso.
                t.1.increment_usage();
                update_totals(t0);
            }
            None => {
                // Se nao existe, adiciona ele e incrementa ou adiciona o RO.
                t0.insert(input.to_string(), Token::new(1));
                let index = unseen.binary_search(&input.as_str()).unwrap();
                unseen.remove(index);
                match t0.iter_mut().find(|item| *item.0 == RO.to_string()) {
                    // Verifica se RO esta em k = 0
                    Some(ro) => {
                        ro.1.increment_usage();
                    }
                    None => {
                        t0.insert(RO.to_string(), Token::new(1));
                    }
                }
                update_totals(t0);
            }
        }
    } else if frase.len() < k {
        return update_tables(frase, k - 1, input, tables, unseen);
    } else if frase.len() == k {
        let tk = tables.get_mut(&k).unwrap();

        // Verifica se a frase existe na tabela k como contexto.
        match tk.iter_mut().find(|context| context.0 == frase) {
            // Se existir, verifica se o input existe dentro do contexto.
            Some(context) => {
                let a = context.1.iter_mut().find(|item| item.0 == input);

                match a {
                    // ‘Input’ existe.
                    Some(t) => {
                        t.1.increment_usage();
                        update_totals(context.1);
                    }
                    // ‘Input’ nao existe.
                    None => {
                        context.1.insert(input.to_string(), Token::new(1));
                        match context.1.iter_mut().find(|item| *item.0 == RO.to_string()) {
                            // Verifica se RO esta em k = 0
                            Some(ro) => {
                                ro.1.increment_usage();
                            }
                            None => {
                                context.1.insert(RO.to_string(), Token::new(1));
                            }
                        }
                        update_totals(context.1);
                        update_tables(&frase[1..].to_string(), k - 1, input, tables, unseen);
                    }
                }
            }
            // A frase nao existe na tabela k
            None => {
                tk.insert(frase.to_string(), ContextItem::new());
                let new_context = tk.get_mut(frase).unwrap();
                new_context.insert(input.to_string(), Token::new(1));
                new_context.insert(RO.to_string(), Token::new(1));
                update_totals(new_context);
                update_tables(&frase[1..].to_string(), k - 1, input, tables, unseen);
            }
        }
    } else if frase.len() > k {
        update_tables(&frase[1..].to_string(), k, input, tables, unseen);
    }
}


pub fn create_tree(tokens: &ContextItem) -> BinaryTree {
    BinaryTree::new(
        tokens
            .iter()
            .map(|t| {
                crate::util::token::Token::new_all(
                    t.0.chars().nth(0).unwrap(),
                    t.1.get_usage(),
                    t.1.get_probability(),
                )
            })
            .collect(),
    )
}

pub fn update_context(context: &mut String, input: char, max: usize) {
    if context.len() < max {
        *context = context.to_owned() + &input.to_string();
    } else {
        *context = context[1..].to_string() + &input.to_string();
    }
}

pub fn exclusion_table(
    tables: &ContextTable,
    context: (&String, &ContextItem),
    k: usize,
) -> Option<ContextTable> {
    if k == 0 {
        return None;
    }
    let mut exclusion = tables.clone();
    let next_frase = context.0[1..].to_string();

    let next_context = exclusion
        .get_mut(&(k - 1))
        .unwrap()
        .get_mut(&next_frase)
        .unwrap();

    context.1.iter().for_each(|item| {
        if *item.0 != RO.to_string() {
            next_context.remove(item.0);
        }
    });
    update_totals(next_context);
    Some(exclusion)
}

