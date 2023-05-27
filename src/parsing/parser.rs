use crate::math::recurrent::Recurrent;

use super::tokenizer::Token;

pub fn parse(tokens: &[Token]) -> Recurrent {
    let mut pos = 0;
    parse_sum(tokens, &mut pos)
}

pub fn parse_general(tokens: &[Token], pos: &mut usize) -> Recurrent {
    let result = match tokens[*pos] {
        Token::OpenBracket => {
            *pos += 1;
            parse_sum(tokens, pos)
        }
        Token::Var => Recurrent::indvar(),
        Token::Const(c) => Recurrent::constant(c),
        _ => panic!("invalid syntax"),
    };
    *pos += 1;
    result
}

pub fn parse_product(tokens: &[Token], pos: &mut usize) -> Recurrent {
    let mut result = parse_general(tokens, pos);
    while *pos < tokens.len() && matches!(tokens[*pos], Token::Mul) {
        *pos += 1;
        let rhs = parse_general(tokens, pos);
        result = result * rhs;
    }
    result
}

pub fn parse_sum(tokens: &[Token], pos: &mut usize) -> Recurrent {
    let mut result = parse_product(tokens, pos);
    while *pos < tokens.len() && matches!(tokens[*pos], Token::Add | Token::Sub) {
        let op = tokens[*pos];
        *pos += 1;
        let rhs = parse_product(tokens, pos);
        if matches!(op, Token::Add) {
            result = result + rhs;
        } else {
            result = result - rhs;
        }
    }
    result
}
