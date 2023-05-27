use std::io::Write;

use crate::parsing::{parser::parse, tokenizer::tokenize};

mod math;
mod parsing;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    loop {
        buf.clear();
        print!("Sum for i in [0; n): ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut buf)?;
        let tokens = tokenize(buf.split_whitespace().flat_map(|s| s.chars()));
        let recurrent = parse(&tokens);
        println!("{}", recurrent.sum_expr());
        println!();
    }
}
