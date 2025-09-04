#![allow(dead_code)]

use crate::parser::ast;

struct RspTracker {
    current: usize,
}

impl RspTracker {
    pub fn new() -> Self {
        Self { current: 0 }
    }

    pub fn next_offset(&mut self, size: usize) -> usize {
        const ALLIGNMENT: usize = 8;
        let old = self.current;
        self.current += ((size + ALLIGNMENT - 1) + 8) & !(ALLIGNMENT - 1);
        old
    }
}


// X86-64 Assembly code generation state
// Nasm Specifically
pub struct State {
    program: Vec<ast::DeclType>,
    extern_buffer: Vec<String>,
    text_buffer: Vec<String>,
    data_buffer: Vec<String>,
    bss_buffer: Vec<String>,

    rsp_tracker: RspTracker,
}

impl State {
    pub fn new(program: Vec<ast::DeclType>) -> Self {
        State { 
            program,
            extern_buffer: Vec::new(),
            text_buffer: Vec::new(),
            data_buffer: Vec::new(),
            bss_buffer: Vec::new(),
            rsp_tracker: RspTracker::new(),
        }
    }

    pub fn generate(&mut self) {
        let program = std::mem::take(&mut self.program);
        for decl in program.iter() {
            self.gen_decl(&decl);
        }
        self.program = program;
        // DEBUG:
        println!("-------------------------------");
        for line in self.extern_buffer.iter() {
            println!("{}", line);
        }
        println!("\nsection .text");
        for line in self.text_buffer.iter() {
            println!("{}", line);
        }
        println!("\nsection .data");
        for line in self.data_buffer.iter() {
            println!("{}", line);
        }
        println!("-------------------------------");
    }

    fn gen_label(&mut self, label: &str) {
        self.text_buffer.push(format!("{}:", label));
    }

    fn gen_inst(&mut self, inst: &str, args: &str) {
        self.text_buffer.push(format!("    {} {}", inst, args));
    }

    fn gen_extern(&mut self, name: &str) {
        self.extern_buffer.push(format!("extern {}", name));
    }

    fn gen_data(&mut self, label: &str, value: &str) {
        self.data_buffer.push(format!("{}: {}", label, value));
    }

    fn gen_func_prologue(&mut self, name: &str) {
        self.gen_label(name);
        self.gen_inst("push", "rbp");
        self.gen_inst("mov", "rbp, rsp");
    }

    fn gen_func_epilogue(&mut self) {
        self.gen_inst("mov", "rsp, rbp");
        self.gen_inst("pop", "rbp");
        self.gen_inst("ret", "");
    }

    fn gen_decl(&mut self, decl: &ast::DeclType) {
        match decl {
             ast::DeclType::FunctionDecl { name, params, body, func_type } => {
                 self.gen_function(&name, &params, &func_type, &body);
             }
             ast::DeclType::VariableDecl { name, var_type, init, mutability } => {
                 self.gen_var_decl(&name, &var_type, &init, &mutability);
             }
             _ => todo!("Code generation for other declaration types not implemented yet"),
        }
    }
    
    fn gen_var_decl(&mut self, name: &str, var_type: &ast::Type, init: &Option<Box<ast::Expr>>, mutability: &ast::Mutability) {
        let offset = self.rsp_tracker.next_offset(32);
        self.gen_inst("mov", &format!("[rbp - {}], 0", offset));

    }

    fn gen_function(&mut self, name: &str, params: &Vec<ast::Parameter>, return_type: &ast::Type, body: &Option<Box<ast::Expr>>) {
        if body.is_none() {
            self.gen_extern(name);
            return;
        }

        self.gen_func_prologue(name);
        let body = body.as_ref().unwrap();
        self.gen_expr(body);
        self.gen_func_epilogue();
    }

    fn gen_expr(&mut self, expr: &ast::Expr) {
        match &*expr {
            ast::Expr::IntegerLiteral(value) => {
                self.gen_inst("mov", &format!("rax, {}", value));
            }
            ast::Expr::BinaryOp { lhs, op, rhs } => {
                self.gen_expr(lhs);
                self.gen_inst("push", "rax");
                self.gen_expr(rhs);
                self.gen_inst("pop", "rbx");
                match op {
                    ast::BinaryOperator::Add => {
                        self.gen_inst("add", "rax, rbx");
                    }
                    ast::BinaryOperator::Sub => {
                        self.gen_inst("sub", "rbx, rax");
                        self.gen_inst("mov", "rax, rbx");
                    }
                    ast::BinaryOperator::Mul => {
                        self.gen_inst("imul", "rax, rbx");
                    }
                    ast::BinaryOperator::Div => {
                        self.gen_inst("xor", "rdx, rdx"); // Clear rdx before div
                        self.gen_inst("mov", "rcx, rax"); // Move divisor to rcx
                        self.gen_inst("mov", "rax, rbx"); // Move dividend to rax
                        self.gen_inst("div", "rcx");       // rax = rax / rcx
                    }
                    _ => todo!("Code generation for other binary operators not implemented yet"),
                }
            }
            ast::Expr::CompoundExpr { expressions } => {
                for decl in expressions.iter() {
                    self.gen_decl(decl);
                }
            }
            _ => todo!("Code generation for other expression types not implemented yet"),
        }
    }
}
