#[derive(Debug, Clone)]
pub struct Token {
    usage: usize,
    total: usize,
}

impl Token {
    pub fn new(usage: usize) -> Self {
        Token { usage, total: 0 }
    }

    // Getters
    pub fn get_usage(&self) -> usize {
        self.usage
    }

    pub fn get_probability(&self) -> f64 {
        self.usage as f64 / self.total as f64
    }

    // Setters
    pub fn set_usage(&mut self, usage: usize) {
        self.usage = usage
    }

    pub fn set_total(&mut self, total: usize) {
        self.total = total;
    }

    pub fn increment_usage(&mut self) {
        self.usage += 1;
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.usage.partial_cmp(&other.usage)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.usage == other.usage
    }
}
