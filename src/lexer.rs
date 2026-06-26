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