use super::lox_type::*;
use super::compiler::*;
use std::vec::IntoIter;

pub struct Vm {
    constants: Vec<LoxType>,
    bytes: IntoIter<u8>
}/*

And, // 8
Or, // 9
Equals, // 10
Greater, // 11
GreaterEqual, // 12
Lesser, // 13
LesserEqual, // 14
*/
impl Vm {
    pub fn new(c: Chunk) -> Vm {
        let bytes = c.encode_ops().into_iter();
        Vm {
            constants: c.constants,
            bytes: bytes
        }
    }

    pub fn do_op(&mut self) -> Result<LoxType,String> {
        match self.bytes.next().ok_or("Weird Error".to_owned())? {
            0 => Err("Return".to_owned()), // RETURN
            1 => Ok(self.constants[  // Constant
                u8::from_le_bytes(
                    [self.bytes.next().ok_or("Weird Error".to_owned())?]
                ) as usize]
                .clone()),
            2 => Ok(self.constants[ // LongConstant
                u16::from_le_bytes(
                    [self.bytes.next().ok_or("Weird Error".to_owned())?
                    ,self.bytes.next().ok_or("Weird Error".to_owned())?]
                ) as usize]
                .clone()),
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
            _ => Err("Invalid Opcode".to_owned())
        }
    }
}
