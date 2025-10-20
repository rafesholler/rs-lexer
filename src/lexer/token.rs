pub struct Token {
    name: String,
    value: String,
}

impl Token {
    pub fn new(name: String, value: String) -> Token {
        Token { name, value }
    }
}