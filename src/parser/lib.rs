use super::token::{Token, TokenType, Span};

#[derive(Debug, Clone)]
pub struct Lexer {
    source: String,
    line:   usize,
    position: usize,
    column:    usize,
    pline:    usize,
    pcol:     usize
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer { source, line: 1, pline: 1, column: 0, pcol: 0, position: 0}
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.position)
    }

    fn eat(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.position += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
        Some(ch)
    }
    
    fn snap_shot(&mut self) {
        self.pline = self.line;
        self.pcol = self.column;
    }

    fn make_token(&mut self, token_type: TokenType) -> Token {
        let span = Span::new(
            self.pline,
            self.line,
            self.pcol,
            self.column - 1
        );
        Token::new(token_type, span)
    }

    pub fn next_token(&mut self) -> Token {
        self.snap_shot();

        let Some(ch) = self.peek() else {
            return self.make_token(TokenType::Eof);
        };
        
        return match ch {
            '0'..='9' => {
                let mut num = 0i64;
                while let Some(digit) = self.peek().filter(|c| c.is_ascii_digit()) {
                    num = num * 10 + (digit as i64 - '0' as i64);
                    self.eat();
                }
                self.make_token(TokenType::IntegerLiteral(num))
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(c) = self.peek().filter(|c| c.is_ascii_alphanumeric() || *c == '_') {
                    ident.push(c);
                    self.eat();
                }
                let token_type = match ident.as_str() {
                    "int" => TokenType::Int,
                    "char" => TokenType::Char,
                    "struct" => TokenType::Struct,
                    "enum" => TokenType::Enum,
                    "return" => TokenType::Return,
                    "if" => TokenType::If,
                    "else" => TokenType::Else,
                    "while" => TokenType::While,
                    "for" => TokenType::For,
                    "break" => TokenType::Break,
                    "continue" => TokenType::Continue,
                    "void" => TokenType::Void,
                    "const" => TokenType::Const,
                    "static" => TokenType::Static,
                    "extern" => TokenType::Extern,
                    "typedef" => TokenType::Typedef,
                    "sizeof" => TokenType::Sizeof,
                    "switch" => TokenType::Switch,
                    "case" => TokenType::Case,
                    "default" => TokenType::Default,
                    "do" => TokenType::Do,
                    "goto" => TokenType::Goto,
                    "union" => TokenType::Union,
                    "signed" => TokenType::Signed,
                    "unsigned" => TokenType::Unsigned,
                    "long" => TokenType::Long,
                    "short" => TokenType::Short,
                    "float" => TokenType::Float,
                    "double" => TokenType::Double,
                    _ => TokenType::Identifier(ident),
                };
                self.make_token(token_type)
            },
            '+' => { self.eat(); self.make_token(TokenType::Add) },
            '-' => { self.eat(); self.make_token(TokenType::Sub) },
            '*' => { self.eat(); self.make_token(TokenType::Mul) },
            '/' => { self.eat(); self.make_token(TokenType::Div) },
            '(' => { self.eat(); self.make_token(TokenType::LParen) },
            ')' => { self.eat(); self.make_token(TokenType::RParen) },
            '{' => { self.eat(); self.make_token(TokenType::LBrace) },
            '}' => { self.eat(); self.make_token(TokenType::RBrace) },
            ';' => { self.eat(); self.make_token(TokenType::SemiColon) },
            ':' => { self.eat(); self.make_token(TokenType::Colon) },
            ',' => { self.eat(); self.make_token(TokenType::Comma) },
            c if c.is_whitespace() => {
                self.eat();
                self.next_token()
            },
            _ => {
                panic!("Unexpected character: {}", ch);
            }
        }
    }
}
