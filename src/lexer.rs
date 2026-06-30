#[derive(Debug)]
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

#[derive(Debug)]
pub struct Lexer {
    text : String,
    pos : usize,
    pub tokens : Vec<Token>,
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
        let len = self.text.chars().count();

        
        while self.pos < len {
            let character = self.text.chars().nth(self.pos);
            match character {
                Some(c) => {
                    match c {
                        c if c.is_whitespace() => {},
                        '[' => self.tokens.push(Token::LBracket),
                        ']' => self.tokens.push(Token::RBracket),
                        '{' => self.tokens.push(Token::LBrace),
                        '}' => self.tokens.push(Token::RBrace),
                        ',' => self.tokens.push(Token::Comma),
                        ':' => self.tokens.push(Token::Colon),
                        'n' => {
                            let suite: String = self.text.chars().skip(self.pos + 1).take(3).collect();
                            if suite == "ull" {
                                self.tokens.push(Token::Null);
                                self.pos += 3;
                            } else {
                                println!("Erreur : attendu 'null'");
                            }
                        }
                        't' => {
                            let suite : String = self.text.chars().skip(self.pos + 1).take(3).collect();
                            if suite == "rue" {
                                self.tokens.push(Token::Bool(true));
                                self.pos += 3;
                            } else {
                                println!("Erreur : attendu 'true'");
                            }
                        }
                        'f' => {
                            let suite : String = self.text.chars().skip(self.pos + 1).take(4).collect();
                            if suite == "alse"  {
                                self.tokens.push(Token::Bool(false));
                                self.pos += 4;
                            } else {
                                println!("Erreur : attendu 'false'");
                            }
                        }
                        '"' => {
                            let  string : String = self.text.chars().skip(self.pos + 1).take_while(|&c| c != '"').collect();
                            self.pos += string.chars().count() + 1; 
                            self.tokens.push(Token::StringToken(string));
                        }
                        '0'..='9' | '-' => {
                            let number : String = self.text.chars().skip(self.pos).take_while(|&c| c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' || c == '+' || c == '-').collect();
                            self.pos += number.chars().count() - 1;

                            if let Ok(parsed_num) = number.parse::<f64>() {
                                self.tokens.push(Token::Number(parsed_num));
                            } else {
                                println!("Erreur : format de nombre invalide '{}'", number);
                            }
                        }
                        _ => {},
                    }
                },
                None => break,
            };
            self.pos += 1;
        }
    }
}