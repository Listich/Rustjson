mod lexer;
use lexer::Lexer;

fn main() {
    let texte_json = String::from(r#"{"a": {"b": 1}}"#);
    let mut lexer = Lexer::new(texte_json);
    println!("Avant tokenize");
    lexer.tokenize();
    println!("Après tokenize");
    println!("Tokens récupérés : {:#?}", lexer.tokens);
}
