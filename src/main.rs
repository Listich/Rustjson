mod lexer;
use lexer::Lexer;
fn main() {
    let texte_json = String::from(r#"{"age": 25, "is_valid": true, "data": null}"#);
    let mut lexer = Lexer::new(texte_json);
    println!("Avant tokenize");
    lexer.tokenize();
    println!("Après tokenize");
    println!("Tokens récupérés : {:#?}", lexer.tokens);
}
