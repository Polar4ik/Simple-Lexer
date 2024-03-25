#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Mul,
    Div,

    NumberLiteral(i64),

    EOF,
}

pub struct Lexer {
    input: String,
    cursor: usize,
}
impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {cursor: 0, input}
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.cursor < self.input.len() {
            if let Some(ch) = self.input.chars().nth(self.cursor) {
                match ch {
                    _ if ch.is_whitespace() => {}, 
                    _ if ch.is_digit(10) => tokens.push(Token::NumberLiteral(number_tokenize(&self.input, &mut self.cursor, ch))),
                    '+' => tokens.push(Token::Plus),
                    '-' => tokens.push(Token::Minus),
                    '*' => tokens.push(Token::Mul),
                    '/' => tokens.push(Token::Div),
                    _ => println!("Err")
                }
            }
            self.cursor += 1;
        }

        tokens.push(Token::EOF);

        return tokens;
    }
}

fn number_tokenize(input: &String, cursor: &mut usize, ch: char) -> i64 {
    let mut num_buffer: String = String::new();
    num_buffer.push(ch);

    let mut chars = input.chars().skip(*cursor + 1).peekable();
    while let Some(nch) = chars.peek() {
        if nch.is_digit(10) {
            num_buffer.push(*nch);
            chars.next();
            *cursor += 1;
        }
        else {
            break;
        }
    }

    return num_buffer.parse::<i64>().unwrap();
}

