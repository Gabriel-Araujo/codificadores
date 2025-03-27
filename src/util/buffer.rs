use super::{WEIGHTS, token::Token};

pub fn construct(input: &String) -> Vec<Token> {
    let mut buffer: Vec<Token> = vec![];
    let input_len = input.len();

    input
        .chars()
        .for_each(|c| match buffer.iter_mut().find(|i| i.get_symbol() == c) {
            Some(t) => t.increment_usage(),
            None => buffer.push(Token::new(c)),
        });

    buffer
        .iter_mut()
        .for_each(|t| t.set_probability(t.get_usage() as f64 / input_len as f64));

    buffer.sort();
    buffer
}

pub fn get_default_weights() -> Vec<Token> {
    let mut buffer: Vec<Token> = WEIGHTS
        .iter()
        .map(|i| {
            let mut temp = Token::new(i.0);
            temp.set_probability(i.1);
            temp
        })
        .collect::<Vec<Token>>();

    buffer.sort();
    buffer
}
