#![allow(dead_code)]

use crate::{diag::diag::Diag, parser};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ParseContext {
    Global,
    Function,
    FunctionParams,
    Block,
    Expression,
}

pub struct Parser {
    token_buffer: Vec<parser::token::Token>,
    position: usize,
    // a program is a list of declarations
    program: Vec<parser::ast::DeclType>,
    // current parsing context
    context: ParseContext,
}

impl Parser {
    pub fn new(tb: Vec<parser::token::Token>) -> Self {
        Parser {
            token_buffer: tb,
            position: 0,
            program: Vec::new(),
            context: ParseContext::Global,
        }
    }

    fn set_context(&mut self, ctx: ParseContext) -> ParseContext {
        let old_ctx = self.context;
        self.context = ctx;
        old_ctx
    }

    fn peek(&self) -> Option<&parser::token::Token> {
        self.token_buffer.get(self.position)
    }

    fn eat(&mut self) {
        self.position += 1;
    }

    fn recover_to(&mut self, token_type: parser::token::TokenType) {
        loop {
            let tok = self.peek();
            if (tok.unwrap().matches(&token_type)) || tok.unwrap().matches(&parser::token::TokenType::Eof) {
                break;
            }
            self.eat();
        }
    }

    fn auto_recover(&mut self) {
        let anchor = match self.context {
            ParseContext::Global | ParseContext::Block => parser::token::TokenType::SemiColon,
            _ => unreachable!()
        };
        self.recover_to(anchor);
    }

    fn expect_identifier(&mut self) -> Result<String, Diag> {
        if let Some(token) = self.peek() {
            if token.is_an_identifier() {
                if let parser::token::TokenType::Identifier(name) = token.get_type() {
                    let name_clone = name.clone();
                    self.eat();
                    return Ok(name_clone);
                } else {
                    unreachable!();
                }
            }
        }
        Err(Diag::MissingIdentifier(self.current_span().clone()))
    }

    fn expect(&mut self, expected: parser::token::TokenType) -> bool {
        if let Some(token) = self.peek() {
            if token.matches(&expected) {
                self.eat();
                return true;
            }
        }
        false
    }

    fn current_span(&self) -> &parser::token::Span {
       self.token_buffer[0].get_span()
    }

    fn match_and<F>(&mut self, expected: parser::token::TokenType, f: F) -> bool
    where
        F: Fn(&parser::token::Token) -> bool,
    {
        if let Some(token) = self.peek() {
            if token.matches(&expected) && f(token) {
                self.eat();
                return true;
            }
        }
        false
    }

    pub fn parse_program(&mut self) -> Result<&Vec<parser::ast::DeclType>, Vec<Diag>> {
        let mut errors = Vec::new();
        while let Some(token) = self.peek() {
            if token.matches(&parser::token::TokenType::Eof) {
                break;
            }
            match self.parse_declaration() {
                Ok(decl) => self.program.push(decl),
                Err(e) => {
                    self.auto_recover();
                    errors.push(e);
                }
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }
        
        Ok(&self.program)
    }

    fn parse_declaration(&mut self) -> Result<parser::ast::DeclType, Diag> {
        let tok = self.peek().unwrap();
        // tokens that can start a declaration:
        // * 'struct': struct definitions
        // * 'enum'  : enumerations
        // * any id  : marco or type name { function | variable_decl }
        // * '#'     : directives
        
        // covering only 'any id' case for now
        if tok.is_a_name() {
            return self.parse_function_or_variable_decl();
        }
        todo!();
    }

    fn parse_function_or_variable_decl(&mut self) -> Result<parser::ast::DeclType, Diag> {
        let type_name = self.parse_type()?;
        let decl_name = match self.peek() {
            Some(t) if t.is_an_identifier() => {
                if let parser::token::TokenType::Identifier(n) = t.get_type() {
                    n.clone()
                } else {
                    unreachable!();
                }
            },
            _ => {
                return Err(Diag::DeclarationMissingAName(self.current_span().clone()));
            }
        };
        self.eat(); // consume the identifier
        // this could be a '(' then we attempt to parse a function
        // or a ';' then we parse a variable forward decl or '=' then we parse
        // a full variable decl

        if self.match_and(parser::token::TokenType::LParen, |_| true) {
            return self.parse_function_decl(type_name, decl_name);
        }

        todo!();
    }

    fn parse_function_decl(&mut self, return_type: Box<parser::ast::Type>, name: String) -> Result<parser::ast::DeclType, Diag> {
        let old_ctx = self.set_context(ParseContext::FunctionParams);
        let mut params = Vec::new();
        self.eat(); // consume '(' 
        while !self.match_and(parser::token::TokenType::RParen, |_| true) {
            // only support named parameters passed by value for now
            let param_type = self.parse_type()?;
            let param_name = self.expect_identifier()?;
            params.push(parser::ast::Parameter::new_named(param_name, param_type, parser::ast::TakeType::ByValue));
        }
        self.set_context(old_ctx);
        // now expect either a ';' for forward declaration or a '{' for function body
        if self.match_and(parser::token::TokenType::SemiColon, |_| true) {
            return Ok(parser::ast::DeclType::FunctionDecl {
                name,
                func_type: return_type, // this we will need to construct a full function type
                                        // later
                params,
                body: None,
            });
        } else if self.match_and(parser::token::TokenType::LBrace, |_| true) {
            // parse function body
            let body = self.parse_expression()?;
            return Ok(parser::ast::DeclType::FunctionDecl {
                name,
                func_type: return_type, // this we will need to construct a full function type
                                        // later
                params,
                body: Some(body),
            });
        }
        todo!();
    }

    // the classic RD expression chain
    fn parse_expression(&mut self) -> Result<Box<parser::ast::Expr>, Diag> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Box<parser::ast::Expr>, Diag> {
        let mut left = self.parse_logical_and()?;
        while self.match_and(parser::token::TokenType::Or, |_| true) {
            let right = self.parse_logical_and()?;
            left = Box::new(parser::ast::Expr::BinaryOp {
                op: parser::ast::BinaryOperator::Or,
                lhs: left,
                rhs: right,
            });
        }
        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<Box<parser::ast::Expr>, Diag> {
        let mut left = self.parse_equality()?;
        while self.match_and(parser::token::TokenType::And, |_| true) {
            let right = self.parse_equality()?;
            left = Box::new(parser::ast::Expr::BinaryOp {
                op: parser::ast::BinaryOperator::And,
                lhs: left,
                rhs: right,
            });
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Box<parser::ast::Expr>, Diag> {
        let mut left = self.parse_relational()?;
        while let Some(tok) = self.peek() {
            if tok.matches(&parser::token::TokenType::Eq) || tok.matches(&parser::token::TokenType::Neq) {
                let op = if tok.matches(&parser::token::TokenType::Eq) {
                    parser::ast::BinaryOperator::Eq
                } else {
                    parser::ast::BinaryOperator::Neq
                };
                self.eat();
                let right = self.parse_relational()?;
                left = Box::new(parser::ast::Expr::BinaryOp {
                    op,
                    lhs: left,
                    rhs: right,
                });
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_relational(&mut self) -> Result<Box<parser::ast::Expr>, Diag> {
        let mut left = self.parse_additive()?;
        while let Some(tok) = self.peek() {
            let op = if tok.matches(&parser::token::TokenType::Lt) {
                parser::ast::BinaryOperator::Lt
            } else if tok.matches(&parser::token::TokenType::Gt) {
                parser::ast::BinaryOperator::Gt
            } else if tok.matches(&parser::token::TokenType::Leq) {
                parser::ast::BinaryOperator::Leq
            } else if tok.matches(&parser::token::TokenType::Geq) {
                parser::ast::BinaryOperator::Geq
            } else {
                break;
            };
            self.eat();
            let right = self.parse_additive()?;
            left = Box::new(parser::ast::Expr::BinaryOp {
                op,
                lhs: left,
                rhs: right,
            });
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<Box<parser::ast::Expr>, Diag> {
        let mut left = self.parse_multiplicative()?;
        while let Some(tok) = self.peek() {
            let op = if tok.matches(&parser::token::TokenType::Add) {
                parser::ast::BinaryOperator::Add
            } else if tok.matches(&parser::token::TokenType::Sub) {
                parser::ast::BinaryOperator::Sub
            } else {
                break;
            };
            self.eat();
            let right = self.parse_multiplicative()?;
            left = Box::new(parser::ast::Expr::BinaryOp {
                op,
                lhs: left,
                rhs: right,
            });
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<Box<parser::ast::Expr>, Diag> {
        let mut left = self.parse_unary()?;
        while let Some(tok) = self.peek() {
            let op = if tok.matches(&parser::token::TokenType::Mul) {
                parser::ast::BinaryOperator::Mul
            } else if tok.matches(&parser::token::TokenType::Div) {
                parser::ast::BinaryOperator::Div
            } else if tok.matches(&parser::token::TokenType::Mod) {
                parser::ast::BinaryOperator::Mod
            } else {
                break;
            };
            self.eat();
            let right = self.parse_unary()?;
            left = Box::new(parser::ast::Expr::BinaryOp {
                op,
                lhs: left,
                rhs: right,
            });
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Box<parser::ast::Expr>, Diag> {
        if let Some(tok) = self.peek() {
            if tok.matches(&parser::token::TokenType::Sub) {
                self.eat();
                let expr = self.parse_unary()?;
                return Ok(Box::new(parser::ast::Expr::UnaryOp {
                    op: parser::ast::UnaryOperator::Neg,
                    expr,
                }));
            } else if tok.matches(&parser::token::TokenType::Not) {
                self.eat();
                let expr = self.parse_unary()?;
                return Ok(Box::new(parser::ast::Expr::UnaryOp {
                    op: parser::ast::UnaryOperator::Not,
                    expr,
                }));
            } else if tok.matches(&parser::token::TokenType::Mul) {
                self.eat();
                let expr = self.parse_unary()?;
                return Ok(Box::new(parser::ast::Expr::UnaryOp {
                    op: parser::ast::UnaryOperator::Deref,
                    expr,
                }));
            } else if tok.matches(&parser::token::TokenType::And) {
                self.eat();
                let expr = self.parse_unary()?;
                return Ok(Box::new(parser::ast::Expr::UnaryOp {
                    op: parser::ast::UnaryOperator::AddrOf,
                    expr,
                }));
            }
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Box<parser::ast::Expr>, Diag> {
        // smallest unit of expressions
        if let Some(tok) = self.peek()  {
            if tok.is_an_integer_literal() {
                if let parser::token::TokenType::IntegerLiteral(val) = tok.get_type() {
                    let value: i64 = *val;  // goddamn rust borrow checker
                    self.eat();
                    return Ok(Box::new(parser::ast::Expr::IntegerLiteral(value)));
                } else {
                    unreachable!();
                }
            }

            if tok.is_an_identifier() {
                if let parser::token::TokenType::Identifier(name) = tok.get_type() {
                    let value = name.clone();
                    self.eat();
                    return Ok(Box::new(parser::ast::Expr::Identifier(value)))
                }
            }

            todo!();
        } else {
            return Err(Diag::EarlyEOF(self.current_span().clone())); 
        }
    }

    fn parse_type(&mut self) -> Result<Box<parser::ast::Type>, Diag> {
        let Some(tok) = self.peek() else {
            return Err(Diag::EarlyEOF(self.current_span().clone()));
        };

        if tok.matches(&parser::token::TokenType::Int) {
            self.eat();
            return Ok(parser::ast::Type::new_cint());
        } else if tok.matches(&parser::token::TokenType::Char) {
            self.eat();
            return Ok(parser::ast::Type::new_cchar());
        } else if tok.matches(&parser::token::TokenType::Void) {
            self.eat();
            return Ok(parser::ast::Type::new_cvoid());
        } else if tok.is_an_identifier() {
            let name = if let parser::token::TokenType::Identifier(n) = tok.get_type() {
                n
            } else {
                unreachable!();
            };
            let mut type_name = Box::new(parser::ast::Type::TypeName(name.clone()));
            self.eat();
            while let Some(t) = self.peek() {
                if t.matches(&parser::token::TokenType::Mul) {
                    type_name = parser::ast::Type::new_pointer(type_name);
                    self.eat();
                } else { break; }
            }
            return Ok(type_name);
        }
        todo!("{:?}", tok.display()); 
    }
}
