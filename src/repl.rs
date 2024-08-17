use std::io::Write;

use crate::lexer;

const PROMPT: &str = "λ> ";

pub fn repl() -> Result<(), std::io::Error> {
    loop {
        print!("{PROMPT}");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        for token in lexer::Lexer::new(input) {
            println!("{token:?}");
        }
    }
}
