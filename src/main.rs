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
use bytecode::emoji;
use std::panic;

use std::rc::Rc;
use std::time::*;

fn main() {
    let og = "var n = 0; while (n < 10) { n = n + 1; } var b = 0;".to_string();
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
    let s = emoji::encode_ops(compiler.ops.clone());
    println!("{}",s);
    println!("{:?}",emoji::decode_ops(s.clone()));
    println!("{:?}",compiler.encode_ops());
    // let time = SystemTime::now();
    panic::catch_unwind(|| { interpret(emoji::decode_ops(s),compiler.constants); });
    // println!("Ran for: {}",time.elapsed().unwrap().as_millis());
}
