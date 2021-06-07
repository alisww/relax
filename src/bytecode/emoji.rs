use super::compiler::*;
use super::compiler::Operation::*;

pub fn encode_ops(ops: Vec<Operation>) -> String {
    let mut res = String::new();
    for op in ops {
        res.push(match op {
            Return => '🔚',
            Constant => '🧱',
            LongConstant => '🔧',
            Add => '➕',
            Subtract => '➖',
            Multiply => '❌',
            Divide => '➗',
            Negate => '❗',
            And => '✨',
            Or => '🥺',
            Equals => '😐',
            Greater => '😌',
            GreaterEqual => '📈',
            Lesser => '😔',
            LesserEqual => '📉',
            Var => '🛹',
            Assign => '📝',
            Pop => '📖',
            Get => '📚',
            JumpBackIfFalse => '🔙',
            JumpBackIfTrue => '🦘',
            JumpIfTrue => '🔛',
            JumpIfFalse => '🔜',
            NotEquals => '😭',
            Operand(i) => unsafe { char::from_u32_unchecked(i as u32) }
        });
    }
    res
}

pub fn decode_ops(ops: String) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();
    for op in ops.chars() {
        res.push((match op {
             '🔚' => 0,
             '🧱' => 1,
             '🔧' => 2,
             '➕' => 3,
             '➖' => 4,
             '❌' => 5,
             '➗' => 6,
             '❗' => 7,
             '✨' => 8,
             '🥺' => 9,
             '😐' => 10,
             '😌' => 11,
             '📈' => 12,
             '😔' => 13,
             '📉' => 14,
             '🛹' => 15,
             '📝' => 16,
             '📖' => 17,
             '📚' => 18,
             '🔙' => 20,
             '🦘' => 19,
             '🔛' => 21,
             '🔜' => 22,
             '😭' => 23,
            _ => (op as u32) as u8
        }).to_le_bytes()[0]);
    }
    res
}
