#[derive(Debug, Clone, Copy)]
pub enum Token {
    Const(i64),
    Var,
    Add,
    Sub,
    Mul,
    OpenBracket,
    CloseBracket,
}

pub fn tokenize(input: impl Iterator<Item = char>) -> Vec<Token> {
    let mut result = Vec::new();
    let mut num: Option<i64> = None;
    for c in input {
        if let Some(digit) = c.to_digit(10) {
            num = match num {
                None => Some(digit as i64),
                Some(num) => Some(10 * num + digit as i64),
            };
            continue;
        }
        if let Some(n) = num {
            result.push(Token::Const(n));
            num = None;
        }
        result.push(match c {
            'i' => Token::Var,
            '+' => Token::Add,
            '-' => Token::Sub,
            '*' => Token::Mul,
            '(' => Token::OpenBracket,
            ')' => Token::CloseBracket,
            c => panic!("unknown symbol: '{}'", c),
        });
    }
    if let Some(num) = num {
        result.push(Token::Const(num));
    }
    result
}
