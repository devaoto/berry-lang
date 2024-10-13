mod lexer;
mod parser;
mod interpreter;

use interpreter::Interpreter;
use lexer::tokenize;
use parser::Parser;

fn main() {
    let input = r#"print(100/3)"#;

    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);

    let ast = parser.parse();
    let mut interpreter = Interpreter::new();

    println!("{:?}", interpreter.interpret(&ast));
}
