#![allow(dead_code)]

use crate::{diag::diag::Diag, parser::{self, token::TokenType}};

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

    pub fn parse_program(&mut self) -> Result<Vec<parser::ast::DeclType>, Vec<Diag>> {
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
        
        Ok(std::mem::take(&mut self.program))
    }

    fn parse_declaration(&mut self) -> Result<parser::ast::DeclType, Diag> {
        // tokens that can start a declaration:
        // * 'struct': struct definitions
        // * 'enum'  : enumerations
        // * 'fn'    : function definitions
        // * '#'     : directives
        
        let Some(tok) = self.peek() else {
            return Err(Diag::EarlyEOF(self.current_span().clone()))
        };
        
        match tok.get_type() {
            parser::token::TokenType::Fn => {
                return self.parse_function_decl();
            }
            parser::token::TokenType::Var => {
                return self.parse_variable_decl();
            }
            _ => {
                let expr = self.parse_expression()?;
                return Ok(parser::ast::DeclType::SideEffect(*expr));
            } 
        }
    }

    fn parse_function_decl(&mut self) -> Result<parser::ast::DeclType, Diag> {
        self.eat();  // eat 'fn'
        let function_name = self.expect_identifier()?;

        self.expect(parser::token::TokenType::LParen);

        let mut params = vec![];
        while !self.match_and(parser::token::TokenType::RParen, |_| true) {
            let param_name = self.expect_identifier()?;
            self.expect(parser::token::TokenType::Colon); 
            let param_type = self.parse_type()?;
            params.push(parser::ast::Parameter::new_named(param_name, param_type, parser::ast::TakeType::ByValue));
            if self.match_and(parser::token::TokenType::Comma, |_| true) {
                self.eat();
            }
        }

        self.expect(parser::token::TokenType::RParen);
        let mut function_type = parser::ast::Type::new_void();
        if self.match_and(parser::token::TokenType::RArrow, |_| true) {
            self.eat();
            function_type = self.parse_type()?;
        }

        if self.match_and(parser::token::TokenType::SemiColon, |_| true) {
            self.eat();
            return Ok(parser::ast::DeclType::FunctionDecl { 
                name: function_name,
                func_type: function_type,
                params, body: None 
            })
        }
        
        // must be a CompoundExpr
        let function_body = self.parse_expression()?;
        Ok(parser::ast::DeclType::FunctionDecl { name: function_name, func_type:
            function_type, params, body: Some(function_body) })
    }

    fn parse_variable_decl(&mut self) -> Result<parser::ast::DeclType, Diag> {
        let is_const = if let Some(tok) = self.peek() {
            if *tok.get_type() == TokenType::Const {
                true
            } else {
                false
            }
        } else {
            return Err(Diag::EarlyEOF(self.current_span().clone()))
        };
        self.eat();

        let variable_name = self.expect_identifier()?;
        // types are a must for now 
        self.expect(TokenType::Colon);
        let variable_type = self.parse_type()?;
        self.expect(TokenType::Eq);
        let initializer = self.parse_expression()?;

        self.expect(TokenType::SemiColon);
        Ok(parser::ast::DeclType::VariableDecl {
            name: variable_name,
            var_type: variable_type,
            init: Some(initializer),
            mutability: if is_const { parser::ast::Mutability::Immutable } else { parser::ast::Mutability::Mutable },
        })
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

            if tok.matches(&parser::token::TokenType::LBrace) {
                self.eat();
                let mut exprs = vec![];
                while !self.match_and(parser::token::TokenType::RBrace, |_| true) {
                    exprs.push(Box::new(self.parse_declaration()?));
                    if self.peek().is_none() {
                       return Err(Diag::EarlyEOF(self.current_span().clone()));
                    }
                }
                self.expect(parser::token::TokenType::RBrace);
                return Ok(Box::new(parser::ast::Expr::CompoundExpr { expressions: exprs }));
            }



            todo!("{:?}", tok.display());
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
            return Ok(parser::ast::Type::new_int());
        } else if tok.matches(&parser::token::TokenType::Char) {
            self.eat();
            return Ok(parser::ast::Type::new_char());
        } else if tok.matches(&parser::token::TokenType::Void) {
            self.eat();
            return Ok(parser::ast::Type::new_void());
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
