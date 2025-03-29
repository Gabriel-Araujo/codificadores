#[derive(Debug, Clone)]
pub struct Token {
    code: String,
    symbol: char,
    usage: usize,
    probability: f64,
}

impl Token {
    pub fn new(symbol: char) -> Self {
        Token {
            code: "".to_string(),
            symbol,
            usage: 1,
            probability: 0.0,
        }
    }

    pub fn new_code(token: &Token, code: &String) -> Self {
        Token {
            code: code.clone(),
            symbol: token.symbol,
            usage: token.usage,
            probability: token.probability,
        }
    }

    pub fn new_all(symbol: char, usage: usize, probability: f64) -> Self {
        Token {
            code: "".to_string(),
            symbol,
            usage,
            probability,
        }
    }

    pub fn get_symbol(&self) -> char {
        self.symbol
    }

    pub fn get_code(&self) -> &str {
        &self.code
    }

    pub fn get_usage(&self) -> usize {
        self.usage
    }

    pub fn get_probability(&self) -> f64 {
        self.probability
    }

    pub fn set_probability(&mut self, probability: f64) {
        self.probability = probability;
    }

    pub fn increment_usage(&mut self) {
        self.usage += 1;
    }

    pub fn add_to_code(&mut self, bit: u8) {
        self.code = bit.to_string() + &self.code;
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.probability.partial_cmp(&other.probability)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.probability == other.probability
    }
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.probability.total_cmp(&other.probability)
    }
}

impl Eq for Token {}
