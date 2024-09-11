use std::io::Write;

mod ast;
mod evaluator;
mod lexer;
mod parser;

const PROMPT: &str = "λ> ";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, this is the Monkey programming language!");
    println!("Let's get started!\n");

    let mut environment = evaluator::environment::Environment::new();

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

        let evaluated = evaluator::eval_program(&program, &mut environment);
        if let Some(evaluated) = evaluated {
            println!("{}", evaluated.inspect());
        }
    }
}
