#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    TypeName(String), // for typedefs or named types
    CxInteger {
        bits: u8,
        signed: bool,
    },
    // 'int', 'char', 'void' from c
    Int,
    Void,
    Char,
    // struct type
    CompoundType {
        name: String,
        fields: Vec<(String, FieldMeta)>,
    },
    // regular pointer type
    // *int, **char, *struct A for standard Cx
    PointerType {
        to: Box<Type>,
    },
    // Reference type
    RefType {
        to: Box<Type>,
        // could be &T or &mut T,
        mutable: bool,
    },
    // Array type
    // [int; 10], [char], [struct A; 5] for standard Cx (also dynamic)
    ArrayType {
        of: Box<Type>,
        length: Option<usize>,
    },
    // Function type
    FunctionType {
        return_type: Box<Type>,
        param_types: Vec<Type>,
        variadic: bool,
        variadic_type: Option<Box<Type>>, // could be optionally typed to constrain variadic args
    },
}

pub enum TakeType {
    ByValue,
    ByRef { mutable: bool },
}

pub struct Parameter {
    name: Option<String>,
    ptype: Box<Type>,
    take_type: TakeType,
}

pub enum DeclType {
    FunctionDecl {
        name: String,
        func_type: Box<Type>,
        params: Vec<Parameter>,
        body: Option<Box<Expr>>, // None for forward declaration
    },
    VariableDecl {
        name: String,
        var_type: Box<Type>,
        init: Option<Box<Expr>>, // None for uninitialized
        mutability: Mutability,
    },
    SideEffect(Expr),
}

pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Eq,
    Neq,
    Lt,
    Gt,
    Leq,
    Geq,
    Assign,
}

pub enum UnaryOperator {
    Neg,    // -expr
    Not,    // !expr
    Deref,  // *expr
    AddrOf, // &expr
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mutability {
    Mutable,
    Immutable,
}

pub enum Expr {
    IntegerLiteral(i64),
    Identifier(String), 
    Variable(String),
    BinaryOp {
        op: BinaryOperator,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOperator,
        expr: Box<Expr>,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    // Function Bodies, Temporay Scope, etc.
    CompoundExpr {
        expressions: Vec<Box<DeclType>>,
    }
    // TODO: more expression types
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldMeta {
    field_type: Type,
    offset: usize,
}

impl Parameter {
    pub fn new_named(name: String, ptype: Box<Type>, take_type: TakeType) -> Self {
        Parameter { name: Some(name), ptype, take_type }
    }

    pub fn new_unnamed(ptype: Box<Type>, take_type: TakeType) -> Self {
        Parameter { name: None, ptype, take_type }
    } 
}

impl Type {
    pub fn new_integer(bits: u8, signed: bool) -> Box<Self> {
        Box::new(Type::CxInteger { bits, signed })
    }

    pub fn new_int() -> Box<Self> {
        Box::new(Type::Int)
    }

    pub fn new_char() -> Box<Self> {
        Box::new(Type::Char)
    }

    pub fn new_void() -> Box<Self> {
        Box::new(Type::Void)
    }

    pub fn new_pointer(to: Box<Type>) -> Box<Self> {
        Box::new(Type::PointerType { to })
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Type::CxInteger { .. } | Type::Int | Type::Char)
    }
}
