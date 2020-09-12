#[macro_use]
extern crate lazy_static;

mod lox_type;
pub mod syntax;
mod err;
mod bytecode;

use syntax::*;
use lox_type::*;
use err::*;
use syntax::scanner::*;
use syntax::parser::*;
use syntax::statements::*;
use syntax::statements::print_statements;
use bytecode::compiler::*;
use bytecode::vm::*;

use std::rc::Rc;

fn main() {
    let og = "var uwu = 0;\n uwu = 2;".to_string();
    println!("Original Expression: {}", og);
    let mut scanner = Scanner::new(og);
    scanner.scan();
    //println!("Tokens: {:?} \n", scanner.tokens);
    let mut parser = Parser::new(scanner.tokens);
    let statements = parser.parse().unwrap();
    print!("AST: ");
    print_statements(statements.clone());
    let mut compiler = Chunk::new();
    compiler.compile_to_ops(statements);
    println!("\nOperations: {:?}", compiler.ops);
    println!("Bytecode: {:?}", compiler.encode_ops());
/*
    if let Statement::Expression(expr) = (*statements[0]).clone() {
        let mut compiler = Chunk::new();
        compiler.ops = compiler.encode_expr(&expr);
        println!("Operations: {:?}", compiler.ops);
        println!("Bytecode: {:?}", compiler.encode_ops());
        let mut machine = Vm::new(compiler);
        println!("Result: {:?}",machine.do_op());
    }*/
}
