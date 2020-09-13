use super::lox_type::*;
use super::compiler::*;
use std::vec::IntoIter;

#[derive(Debug,Clone)]
pub struct Var {
    idx: u8,
    v: LoxType
}

impl Var {
    pub fn new(l: u8,v:LoxType) -> Var {
        Var {
            idx: l,
            v: v
        }
    }
}

pub struct Vm {
    constants: Vec<LoxType>,
    pub bytes: Vec<u8>,
    pub idx: usize,
    pub stack: Vec<Var>
}

/*

And, // 8
Or, // 9
Equals, // 10
Greater, // 11
GreaterEqual, // 12
Lesser, // 13
LesserEqual, // 14
*/
macro_rules! next {
    ($s:expr) => {
        {
            $s.idx += 1;
            $s.bytes[$s.idx - 1]
        }
    }
}

impl Vm {
    pub fn new(c: Chunk) -> Vm {
        Vm {
            bytes: c.encode_ops(),
            constants: c.constants,
            idx: 0,
            stack: Vec::new()
        }
    }

    pub fn do_op(&mut self) -> Result<LoxType,u8> {
        match next!(self) {
            0 => Err(2), // RETURN
            1 => Ok(self.constants[  // Constant
                u8::from_le_bytes(
                    [next!(self)]
                ) as usize].into_owned()),
            2 => Ok(self.constants[ // LongConstant
                u16::from_le_bytes(
                    [next!(self)
                    ,next!(self)]
                ) as usize].into_owned()),
            3 => Ok(self.do_op()? + self.do_op()?), // Add
            4 => Ok(self.do_op()? - self.do_op()?), // Subtract
            5 => Ok(self.do_op()? * self.do_op()?), // Multiply
            6 => Ok(self.do_op()? / self.do_op()?), // Divide
            7 => Ok(!(self.do_op()?)), // Not
            8 => Ok(LoxType::Boolean(self.do_op()?.into() && self.do_op()?.into())), // And
            9 => Ok(LoxType::Boolean(self.do_op()?.into() || self.do_op()?.into())), // Or
            10 => Ok(LoxType::Boolean(self.do_op()? == self.do_op()?)), // Equals
            11 => Ok(LoxType::Boolean(self.do_op()? > self.do_op()?)), // Greater
            12 => Ok(LoxType::Boolean(self.do_op()? >= self.do_op()?)), // GreaterEqual
            13 => Ok(LoxType::Boolean(self.do_op()? < self.do_op()?)), // Lesser Equal
            14 => Ok(LoxType::Boolean(self.do_op()? <= self.do_op()?)), // Lesser Equal
            15 => { // VAR
                let idx = u8::from_le_bytes([next!(self)]);
                self.stack.push(Var::new(idx,LoxType::Nil));
                Ok(LoxType::Nil)
            },
            16 => { // ASSIGN
                let idx = u8::from_le_bytes([next!(self)]);
                if let Some(pos) = self.stack.iter().position(|x| x.idx == idx) {
                    self.stack[pos] = Var::new(idx,self.do_op()?);
                } else {
                    panic!();
                }
                Ok(LoxType::Nil)
            },
            17 => { // POP
                let amt = u8::from_le_bytes([next!(self)]);
                self.stack.truncate(self.stack.len() - amt as usize);
                Ok(LoxType::Nil)
            },
            18 => { // GET
                let idx = u8::from_le_bytes([next!(self)]);
                if let Some(pos) = self.stack.iter().position(|x| x.idx == idx) {
                    Ok(self.stack[pos].v.clone()) // this is bad!
                } else {
                    panic!("uwu");
                }
            },
            19 => { // JumpBackIfTrue
                let should_i = self.do_op()?;
                if bool::from(should_i) {
                    let amt = u8::from_le_bytes([next!(self)]);
                    self.idx -= amt as usize;
                }
                Ok(LoxType::Nil)
            },
            20 => { // JumpBackIfTrue
                let should_i = self.do_op()?;
                if !bool::from(should_i) {
                    let amt = u8::from_le_bytes([next!(self)]);
                    self.idx -= amt as usize;
                }
                Ok(LoxType::Nil)
            }
            21 => { // JumpIfTrue
                let should_i = self.do_op()?;
                if bool::from(should_i) {
                    let amt = u8::from_le_bytes([next!(self)]);
                    self.idx += amt as usize;
                }
                Ok(LoxType::Nil)
            },
            22 => { // JumpIfFalse
                let should_i = self.do_op()?;
                if !bool::from(should_i) {
                    let amt = u8::from_le_bytes([next!(self)]);
                    self.idx += amt as usize;
                }
                Ok(LoxType::Nil)
            },
            _ => Err(1) // NO OP
        }
    }
}
