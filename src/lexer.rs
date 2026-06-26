pub enum Token {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Colon,
    StringToken(String),
    Number(f64),
    Bool(bool),
    Null,
}

pub struct Lexer {
    text : String,
    pos : usize,
    tokens : Vec<Token>,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Lexer {
            text,
            pos: 0,
            tokens: Vec::new(),
        }
    }
    pub fn tokenize(&mut self) {
    }
}