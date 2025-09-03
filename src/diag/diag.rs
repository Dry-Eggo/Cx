#![allow(dead_code)]

use crate::parser::token::Span;
use crate::parser::ast::{Type, Mutability};

#[derive(Debug, Clone)]
pub enum Diag {
    // Lexical Errors
    InvalidCharacter(char, Span),
    UnterminatedString(Span),
    InvalidNumberFormat(String, Span),
    // Syntax Errors
    UnexpectedToken(String, Span),
    MissingToken(String, Span),
    DeclarationMissingAName(Span),
    MissingIdentifier(Span),
    EarlyEOF(Span),
    // Semantic Errors
    UndefinedVariable {
        err_loc: Span,
        var_name: String
    },
    RedefinedVariable {
        err_loc: Span,
        var_name: String,
        prev_decl: Span
    },
    TypeMismatch {
        err_loc: Span,
        expected: Type,
        got: Type,
    },
    ReferenceMutMismatch {
        err_loc: Span,
        expected: Mutability,
        got: Mutability,
    },
    InvalidOperation {
        err_loc: Span,
        operation: String,
        operand_type: Type,
    },
}
