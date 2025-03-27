use crate::util::token::Token;

pub fn generate_table(buffer: Vec<Token>, code: String) -> Vec<Token> {
    let mut temp = generate_table_implementation(buffer, code);

    temp.sort_by_key(|t| {
        (
            (t.get_probability() * 100.0) as usize,
            -(t.get_code().len() as isize),
        )
    });

    temp
}

fn generate_table_implementation(buffer: Vec<Token>, code: String) -> Vec<Token> {
    if buffer.len() == 1 {
        return vec![Token::new_code(&buffer[0], &format!("{code}1"))];
    }
    if buffer.len() == 2 {
        return vec![
            Token::new_code(&buffer[0], &format!("{code}1")),
            Token::new_code(&buffer[1], &format!("{code}0")),
        ];
    }

    let mut input = buffer.clone();
    input.sort();

    let total: f64 = input.iter().map(|token| token.get_probability()).sum();
    let limit = total / 2.0;
    let mut sum: f64 = 0.0;

    let mut right = vec![];
    loop {
        let temp = input.pop().unwrap();
        sum += temp.get_probability();
        right.push(temp);
        if sum + input.last().unwrap().get_probability() > limit {
            break;
        }
    }

    let mut return_value = generate_table_implementation(right, format!("{code}0"));
    return_value.append(&mut generate_table_implementation(
        input,
        format!("{code}1"),
    ));

    return_value
}
