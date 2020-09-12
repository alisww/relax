use super::expr::*;
use super::token_type::*;
use super::lox_type::*;
use super::statements::*;
use std::rc::Rc;

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
    Return, // 0
    Constant, // 1
    LongConstant, // 2
    Add, // 3
    Subtract, // 4
    Multiply, // 5
    Divide, // 6
    Negate, // 7
    And, // 8
    Or, // 9
    Equals, // 10
    Greater, // 11
    GreaterEqual, // 12
    Lesser, // 13
    LesserEqual, // 14
    Var, // 15 declare a variable
    Assign, // 16 Assign a variable
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
    pub constants: Vec<LoxType>
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            ops: Vec::new(),
            constants: Vec::new()
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
            &Expr::Literal(ref v) => {
                ops.extend_from_slice(&self.op_const(v.clone()));
            },
            _ => {}
        }
        ops
    }

    pub fn encode_statement(&mut self,st: Statement) -> Vec<Operation> {
        let mut ops = Vec::new();
        match st {
            Statement::Expression(e) => ops.extend_from_slice(&self.encode_expr(&e)),
            Statement::Variable(name,e) => {
                ops.push(Operation::Var);
                let name_idx = self.set_const(LoxType::String(name.lexeme));
                ops.push(Operation::Operand(name_idx as u64));
                if let Some(expr) = e {
                    ops.push(Operation::Assign);
                    ops.extend_from_slice(&self.encode_expr(&expr));
                    ops.push(Operation::Operand(name_idx as u64));
                }
            },
            _ => {}
        }

        ops
    }

    pub fn encode_ops(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for op in &self.ops {
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
