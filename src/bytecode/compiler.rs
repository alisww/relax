use super::expr::*;
use super::token_type::*;
use super::lox_type::*;
use super::statements::*;
use std::rc::Rc;
use std::collections::HashMap;

trait VecPutAndGetIndex {
    type Item;
    fn put_and_get_index(&mut self, item: Self::Item) -> usize;
}

impl<T> VecPutAndGetIndex for Vec<T> {
    type Item = T;
    fn put_and_get_index(&mut self, item: T) -> usize {
        let idx = self.len();
        self.push(item);
        idx
    }
}

#[derive(Debug,Clone,Copy)]
pub enum Operation {
    Return, // 0 🔚
    Constant, // 1 🧱
    LongConstant, // 2 🔧
    Add, // 3 ➕
    Subtract, // 4 ➖
    Multiply, // 5 ✖️
    Divide, // 6 ➗
    Negate, // 7 ⁉️
    And, // 8 ✨
    Or, // 9 🥺
    Equals, // 10 😐
    Greater, // 11 😌
    GreaterEqual, // 12 📈
    Lesser, // 13 😔
    LesserEqual, // 14 📉
    Var, // 15 declare a variable 🛹
    Assign, // 16 Assign a variable ✍️
    Pop, // 17 Pop from the locals stack, 📖
    Get, // 18 Get value from a variable 📚
    JumpBackIfFalse, // 19 🔙
    JumpBackIfTrue, // 20 ↔️
    JumpIfTrue, // 21 ↗️
    JumpIfFalse, // 22 ↘️
    NotEquals, // 23, 😭
    Operand(u64) // internally it's an u64, but it can range from u8 up to 64
}

/*
RETURN
CONSTANT <OPERAND>
LongConstant <OPERAND>
Add <OPERATION> <OPERATION>
...
NOT <OPERATION>
...
VAR <OPERAND> // id of variable, constants list
ASSIGN <OPERAND> <OP> // index of identifier, value to assign
*/

impl Operation {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            &Operation::Return => bytes.push(0),
            &Operation::Constant => bytes.push(1),
            &Operation::LongConstant => bytes.push(2),
            &Operation::Add => bytes.push(3),
            &Operation::Subtract => bytes.push(4),
            &Operation::Multiply => bytes.push(5),
            &Operation::Divide => bytes.push(6),
            &Operation::Negate => bytes.push(7),
            &Operation::And => bytes.push(8),
            &Operation::Or => bytes.push(9),
            &Operation::Equals => bytes.push(10),
            &Operation::Greater => bytes.push(11),
            &Operation::GreaterEqual => bytes.push(12),
            &Operation::Lesser => bytes.push(13),
            &Operation::LesserEqual => bytes.push(14),
            &Operation::Var => bytes.push(15),
            &Operation::Assign => bytes.push(16),
            &Operation::Pop => bytes.push(17),
            &Operation::Get => bytes.push(18),
            &Operation::JumpBackIfTrue => bytes.push(19),
            &Operation::JumpBackIfFalse => bytes.push(20),
            &Operation::JumpIfTrue => bytes.push(21),
            &Operation::JumpIfFalse => bytes.push(22),
            &Operation::NotEquals => bytes.push(23),
            &Operation::Operand(ref a) => {
                if *a < u8::MAX as u64 {
                    bytes.extend_from_slice(&(*a as u8).to_le_bytes())
                } else if *a < u16::MAX as u64 {
                    bytes.extend_from_slice(&(*a as u16).to_le_bytes())
                } else if *a < u32::MAX as u64 {
                    bytes.extend_from_slice(&(*a as u32).to_le_bytes())
                } else {
                    bytes.extend_from_slice(&(*a as u64).to_le_bytes())
                }
            }
        }
        bytes
    }
}


#[derive(Debug)]
pub struct Chunk {
    pub ops: Vec<Operation>,
    pub constants: Vec<LoxType>,
    var_count: u8,
    curr_depth: u8
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            ops: Vec::new(),
            constants: Vec::new(),
            var_count: 0,
            curr_depth: 0
        }
    }

    pub fn op_const(&mut self,v: LoxType) -> Vec<Operation> {
        let mut ops = Vec::new();
        let idx = self.set_const(v);
        if idx < 256 {
            ops.push(Operation::Constant);
        } else {
            ops.push(Operation::LongConstant);
        }
        ops.push(Operation::Operand(idx as u64));
        ops
    }

    pub fn set_const(&mut self,v: LoxType) -> usize {
        if let Some(_idx) = self.constants.iter().position(|x| x == &v) {
            _idx
        } else {
            self.constants.put_and_get_index(v)
        }
    }

    pub fn encode_expr(&mut self, expr: &Expr) -> Vec<Operation> {
        let mut ops = Vec::new();

        match expr {
            &Expr::Grouping(ref ex) => {
                ops.extend_from_slice(&self.encode_expr(ex));
            },
            &Expr::Assign(ref n, ref ex) => {
                if let Some(idx) = self.constants.iter().position(|x| x == &LoxType::String(n.lexeme.clone())) {
                    ops.push(Operation::Assign);
                    ops.push(Operation::Operand(idx as u64));
                    ops.extend_from_slice(&self.encode_expr(ex));
                }
            },
            &Expr::Unary(_, ref ex) => {
                ops.push(Operation::Negate);
                ops.extend_from_slice(&self.encode_expr(ex));
            },
            &Expr::Binary(ref one, ref token, ref two) => {
                match token.token {
                    TokenType::Plus => {
                        ops.push(Operation::Add);
                    },
                    TokenType::Minus => {
                        ops.push(Operation::Subtract);

                    },
                    TokenType::Star => {
                        ops.push(Operation::Multiply);

                    },
                    TokenType::Slash => {
                        ops.push(Operation::Divide);
                    },
                    _ => {}
                }
                ops.extend_from_slice(&self.encode_expr(one));
                ops.extend_from_slice(&self.encode_expr(two));
            },
            &Expr::Logical(ref one,ref token, ref two) => {
                ops.push(match token.token {
                    TokenType::And => Operation::And,
                    TokenType::Or => Operation::Or,
                    TokenType::EqualEqual => Operation::Equals,
                    TokenType::BangEqual => Operation::NotEquals,
                    TokenType::Greater => Operation::Greater,
                    TokenType::GreaterEqual => Operation::GreaterEqual,
                    TokenType::Less => Operation::Lesser,
                    TokenType::LessEqual => Operation::LesserEqual,
                    _ => Operation::Return
                });
                ops.extend_from_slice(&self.encode_expr(one));
                ops.extend_from_slice(&self.encode_expr(two));
            },
            &Expr::Literal(ref v) => {
                ops.extend_from_slice(&self.op_const(v.clone()));
            },
            &Expr::Variable(ref t) => {
                if let Some(_idx) = self.constants.iter().position(|x| x == &LoxType::String(t.lexeme.clone())) {
                    ops.push(Operation::Get);
                    ops.push(Operation::Operand(_idx as u64)); // idx = u16
                }
            },
            _ => {}
        }
        ops
    }

    pub fn encode_statement(&mut self,st: Statement) -> Vec<Operation> {
        let mut ops = Vec::new();
        ops.reserve(256);
        match st {
            Statement::Expression(e) => ops.extend_from_slice(&self.encode_expr(&e)),
            Statement::Variable(name,e) => {
                let name_idx = self.set_const(LoxType::String(name.lexeme.clone()));
                if self.curr_depth > 0 { self.var_count += 1; };
                ops.push(Operation::Var);
                ops.push(Operation::Operand(name_idx as u64)); // push to stack
                if let Some(expr) = e {
                    ops.push(Operation::Assign);
                    ops.push(Operation::Operand(name_idx as u64));
                    ops.extend_from_slice(&self.encode_expr(&expr));
                }
            },
            Statement::Block(statements) => {
                self.curr_depth += 1;
                for s in statements {
                    let new_ops = self.encode_statement((*s).clone());
                    ops.extend_from_slice(&new_ops);
                }
                if self.var_count > 0 {
                    ops.push(Operation::Pop);
                    ops.push(Operation::Operand(self.var_count as u64));
                }
                self.var_count = 0;
                self.curr_depth -= 1;
            },
            Statement::If(expr,first,else_path) => {
                let expr_ops = self.encode_expr(&expr);
                let if_branch_ops = self.encode_statement((*first).clone());
                ops.push(Operation::JumpIfFalse);
                ops.extend_from_slice(&expr_ops);
                ops.push(Operation::Operand((expr_ops.len() + if_branch_ops.len()) as u64));
                ops.extend_from_slice(&if_branch_ops);
                if let Some(else_branch) = else_path { ops.extend_from_slice(&self.encode_statement((*else_branch).clone())); };
            },
            Statement::While(expr,first) => {
                let expr_ops = self.encode_expr(&expr);
                let block_ops = self.encode_statement((*first).clone());
                println!("{:?}",block_ops);
                ops.extend_from_slice(&block_ops);
                ops.push(Operation::JumpBackIfTrue);
                ops.extend_from_slice(&expr_ops);
                ops.push(Operation::Operand((expr_ops.len() + block_ops.len() + 2) as u64));
            }
            _ => {}
        }

        ops
    }

    pub fn encode_ops(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for op in &self.ops {
    //        println!("{:?}",op);
            bytes.extend_from_slice(&op.to_bytes())
        }
        bytes
    }

    pub fn compile_to_ops(&mut self,st: Vec<Rc<Statement>>) {
        for s in st.into_iter() {
            let enc = self.encode_statement((*s).clone());
            self.ops.extend_from_slice(&enc);
        }
    }
}


/*
while (n < 100) { n = n + 1; }
block = ASSIGN INDEX CONSTANT INDEX
block = 4 bytes
JUMP_BACK_IF_FALSE LESSER GET VAR_INDEX CONSTANT INDEX OFFSET
if n < 100, jump back 4 bytes + amount of bytes in jump expr


*/
