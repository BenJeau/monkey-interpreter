use std::io::Write;

use crate::{lexer, parser};

const PROMPT: &str = "λ> ";

pub fn repl() -> Result<(), std::io::Error> {
    loop {
        print!("{PROMPT}");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let Some(program) = parser.parse_program() else {
            println!("Error parsing program");
            continue;
        };

        if parser.errors.len() > 0 {
            println!("Woops! We ran into some monkey business here!\n");
            println!("Parser errors:");
            for error in parser.errors.iter() {
                println!("- {error}");
            }
            continue;
        }

        println!("{}", program.to_string());
    }
}
