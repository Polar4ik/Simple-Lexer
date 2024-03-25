#![allow(dead_code)]

use crate::lexer::Token;

pub enum Expression {
    NumberExpression(i64),

    BinaryExpression(char, i64, i64),
}

impl Expression {
    pub fn eval(&self) -> i64 {
        match self {
            Expression::BinaryExpression(operation, l, r) => {
                match operation {
                    '+' => return l + r,
                    _ => panic!("Unkown operation")
                }
            },
            _ => panic!("Unkown exp")
        }
    }
} 

pub fn parse(tokens: &Vec<Token>) -> Vec<Expression> {
    let mut stack: Vec<i64> = Vec::new();
    let mut expressions: Vec<Expression> = Vec::new();

    let mut cursor: usize = 0;

    while cursor < tokens.len() {
        let token = tokens.get(cursor).unwrap();
        match token {
            Token::EOF => break,
            Token::NumberLiteral(num) => stack.push(*num),
            Token::Plus => {
                if stack.len() > 1 {
                    panic!("Not number befor plus");
                };
                
                let nt: Option<&Token>  = tokens.get(cursor + 1);
                match nt {
                    Some(Token::NumberLiteral(num)) => {
                        expressions.push(Expression::BinaryExpression('+', stack.pop().unwrap(), *num));
                        cursor += 1;
                    },
                    None => break,
                    _ => panic!("IDK")
                }
            },
           _ => panic!("Unkown token") 
        }
        cursor += 1;
    }

    return expressions;

}
