mod parser;
mod diag;

use parser::parser as p;

fn main() {
    let source = String::from("int main() { return 0; }");
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
    parser.parse_program();
}

