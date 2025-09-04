

// Define the TokenType enum to represent different types of tokens
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Keywords
    Fn, Var,
    Int, Char, Struct, Enum,
    Return, If, Else, While, For, Break,
    Continue, Void, Const, Static, Extern, Typedef, Sizeof,
    Switch, Case, Default, Do, Goto, Union, 

    // values
    Identifier(String),
    IntegerLiteral(i64),

    // operators
    Add,
    Sub,
    Mul,
    Div,
    Eq, Neq,
    Lt, Gt, Leq, Geq,
    Assign,
    And, Or, Not,
    Inc, Dec,
    AddrOf, Deref,
    Mod, Xor,
    Shl, Shr,

    // punctuations
    LParen, RParen,
    LBrace, RBrace,
    SemiColon, Colon, 
    Comma, 
    LArrow, RArrow,

    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
   start_line: usize,
   end_line:   usize,
   start_col:  usize,
   end_col:    usize,
}

impl Span {
    pub fn new(start_line: usize, end_line: usize, start_col: usize, end_col: usize) -> Self {
        Span { start_line, end_line, start_col, end_col }
    }

    pub fn merge(&self, other: &Span) -> Self {
        Span {
            start_line: self.start_line,
            end_line: other.end_line,
            start_col: self.start_col,
            end_col: other.end_col,
        }
    }
}

pub struct Token {
    token_type: TokenType,
    span: Span,
}

impl Token {
    pub fn new(token_type: TokenType, span: Span) -> Self {
        Token { token_type, span }
    }

    pub fn display(&self) -> String {
        match &self.token_type {
            TokenType::Fn  => "fn".to_string(),
            TokenType::Var => "var".to_string(),
            TokenType::Const => "const".to_string(),
            TokenType::Int => "int".to_string(),
            TokenType::Char => "char".to_string(),
            TokenType::Struct => "struct".to_string(),
            TokenType::Enum => "enum".to_string(),
            TokenType::Identifier(name) => format!("identifier({})", name),
            TokenType::IntegerLiteral(value) => format!("integer({})", value),
            TokenType::Add => "+".to_string(),
            TokenType::Sub => "-".to_string(),
            TokenType::Mul => "*".to_string(),
            TokenType::Div => "/".to_string(),
            TokenType::LParen => "(".to_string(),
            TokenType::RParen => ")".to_string(),
            TokenType::LBrace => "{".to_string(),
            TokenType::RBrace => "}".to_string(),
            TokenType::SemiColon => ";".to_string(),
            TokenType::Colon => ":".to_string(),
            TokenType::Comma => ",".to_string(),
            TokenType::Eq => "=".to_string(),
            TokenType::Return => "return".to_string(),
            TokenType::If => "if".to_string(),
            TokenType::Else => "else".to_string(),
            TokenType::While => "while".to_string(),
            TokenType::For => "for".to_string(),
            TokenType::Break => "break".to_string(),
            TokenType::LArrow => "->".to_string(),
            TokenType::RArrow => "<-".to_string(),
            TokenType::Eof => "EOF".to_string(),
            _ => todo!(),
        }
    }

    pub fn get_span(&self) -> &Span {
        &self.span
    }

    pub fn matches(&self, other: &TokenType) -> bool {
        return self.token_type == *other;
    }

    pub fn is_a_name(&self) -> bool {
        return match self.token_type {
            TokenType::Identifier(_) => true,
            // other possible type names will be covered later
            TokenType::Int | TokenType::Char | TokenType::Void | TokenType::Struct | TokenType::Enum => true,
            _ => false,
        }
    }

    pub fn is_an_identifier(&self) -> bool {
        return match self.token_type {
            TokenType::Identifier(_) => true,
            _ => false,
        }
    }

    pub fn is_an_integer_literal(&self) -> bool {
        return match self.token_type {
            TokenType::IntegerLiteral(_) => true,
            _ => false,
        }
    }

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }
}
