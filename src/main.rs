pub mod lexer;
pub mod parser;

fn main() {
    let input: String = "100 + 155".to_string();

    let base_lexer: lexer::Lexer = lexer::Lexer::new(input);
    let tokens: Vec<lexer::Token> = base_lexer.tokenize();

    let exp: Vec<parser::Expression> = parser::parse(&tokens);

    println!("{:?}", tokens);
    println!("{:?}", exp.last().unwrap().eval());
}