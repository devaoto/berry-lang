mod lexer;
mod parser;
use lexer::tokenize;
use parser::Parser;

fn main() {
    let input = r#"a = 2
    "#;

    let tokens = tokenize(input);
    let mut parser = Parser::new(tokens);

    let ast = parser.parse();
    println!("{:?}", ast);
}
