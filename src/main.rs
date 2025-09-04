mod ir;
mod parser;
mod diag;
mod codegen;

use parser::parser as p;

use crate::codegen::codegen::State;

fn main() {
    let source = String::from("fn main() {\n var foo: int = 40;\n var ahh : int = 50;\n}");
    let mut lexer = parser::lib::Lexer::new(source);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        println!("{:?}", token.display());
        if token.matches(&parser::token::TokenType::Eof) {
            break;
        }
        tokens.push(token);
    }
    let mut parser = p::Parser::new(tokens);
    let ast = parser.parse_program().unwrap();
    State::new(ast).generate(); 
}

