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
use std::time::*;

fn main() {
    let og = "var n = 0; while (n < 900000000) { n = n + 1; }".to_string();
    println!("Original Expression: {}", og);
    let mut scanner = Scanner::new(og);
    scanner.scan();
    //println!("Tokens: {:?} \n", scanner.tokens);
    let mut parser = Parser::new(scanner.tokens);
    let statements = parser.parse().unwrap();
    //print!("AST: {:?}\n",statements);
    let mut compiler = Chunk::new();
    compiler.compile_to_ops(statements);
    println!("Operations: {:?}", compiler.ops);
//    println!("Bytecode: {:?}", compiler.encode_ops());
    let mut machine = Vm::new(compiler);
    let now = SystemTime::now();
    let mut iters = 0;
    while machine.idx < machine.bytes.len() - 6 {
        machine.do_op();
    }
    println!("{}",now.elapsed().unwrap().as_millis());
    println!("{:?}",iters);
}
