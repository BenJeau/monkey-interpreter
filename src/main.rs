mod ast;
mod lexer;
mod parser;
mod repl;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, this is the Monkey programming language!");
    println!("Let's get started!\n");

    repl::repl()?;

    Ok(())
}
