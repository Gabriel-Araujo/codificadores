use std::{
    env::args,
    fs::File,
    io::{self, Read, Write},
    process::exit,
};

pub fn get_file_content() -> String {
    let mut content = String::new();

    let file_path = match args().find(|i| i.starts_with("--path=")) {
        Some(t) => t
            .split("--path=")
            .nth(1)
            .expect("An error ocurred.")
            .to_string(),
        None => {
            eprintln!("An error ocurred. \nFile path should be passed.");
            exit(0x57)
        }
    };

    let file = File::open(file_path);

    match file {
        Err(_) => {
            eprintln!("File not Found.");
            exit(0x2);
        }
        Ok(mut t) => {
            t.read_to_string(&mut content)
                .expect("Could not read file.");
        }
    };

    let content = content
        .to_lowercase()
        .chars()
        .map(change_to_ascii)
        .filter(|c| {
            !c.is_numeric()
                && !c.is_whitespace()
                && !c.is_ascii_punctuation()
                && c.is_ascii_alphabetic()
        })
        .collect();

    write_raw_out(&content).expect("Failed to write raw output file");

    content
}

pub fn get_other_vars() -> Vec<String> {
    args().collect::<Vec<String>>()[1..].to_vec()
}

fn change_to_ascii(c: char) -> char {
    match c {
        'â' | 'á' | 'à' | 'ã' => 'a',
        'ê' | 'é' | 'è' => 'e',
        'î' | 'í' | 'ì' => 'i',
        'ô' | 'ó' | 'ò' | 'õ' => 'o',
        'û' | 'ú' | 'ù' | 'ü' => 'u',
        'ñ' => 'n',
        'ç' => 'c',
        '“' | '”' | '’' | '—' | '–' | 'º' => ' ',
        t => t,
    }
}

fn write_raw_out(content: &String) -> io::Result<()> {
    let mut f = File::create("raw_out.txt")?;

    content
        .as_bytes()
        .iter()
        .for_each(|c| write!(f, "{c:b}").expect("Failed to write raw output"));

    Ok(())
}
