use super::lox_type::*;

static mut idx: usize = 0;

macro_rules! read {
    ($b:expr) => {
        {
            unsafe { idx = idx + 1 };
            unsafe { $b[idx - 1] }
        }
    }
}

macro_rules! read_u8 {
    ($b:expr) => {
        {
            u8::from_le_bytes([read!($b)])
        }
    }
}

macro_rules! read_u16 {
    ($b:expr) => {
        {
            u16::from_le_bytes([read!($b),read!($b)])
        }
    }
}

type VmRes = LoxType;

macro_rules! op {
    ($f:expr) => {
        $f
    }
}

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


// new index, value
const OPS: [fn(v: &Vec<u8>, constants: &Vec<LoxType>) -> VmRes; 24] = [
    op!(return_op),
    op!(constant_op),
    op!(long_constant_op),
    op!(add_op),
    op!(sub_op),
    op!(mul_op),
    op!(div_op),
    op!(not_op),
    op!(and_op),
    op!(or_op),
    op!(equals_op),
    op!(greater_op),
    op!(greater_equal_op),
    op!(lesser_op),
    op!(lesser_equal_op),
    op!(var_op),
    op!(assign_op),
    op!(pop_op),
    op!(get_op),
    op!(jump_back_if_true_op),
    op!(jump_back_if_false_op),
    op!(jump_if_true_op),
    op!(jump_if_false_op),
    op!(not_equals_op)
];

static mut stack: Vec<Var> = Vec::new();

fn stack_lookup(_idx: u8) -> Option<usize> {
    unsafe { stack.iter().position(|x| x.idx == _idx) }
}

pub fn interpret(bytes: Vec<u8>,constants: Vec<LoxType>) {
    unsafe {
        while idx < bytes.len() {
            do_op(&bytes,&constants);
        }
    }
    unsafe { println!("Stack after run: {:?}",stack) };
}

fn do_op(bytes: &Vec<u8>, constants: &Vec<LoxType>) -> VmRes {
    OPS[read_u8!(bytes) as usize](bytes,constants)
}

fn return_op(bytes: &Vec<u8>, constants: &Vec<LoxType>) -> VmRes {
    LoxType::Nil
}

fn constant_op(b: &Vec<u8>, c: &Vec<LoxType>) -> VmRes {
    c[read_u8!(b) as usize].into_owned()
}

fn long_constant_op(b: &Vec<u8>, c: &Vec<LoxType>) -> VmRes{
    c[read_u16!(b) as usize].into_owned()
}

fn add_op(b: &Vec<u8>, c: &Vec<LoxType>) -> VmRes {
    do_op(b,c) + do_op(b,c)
}

fn sub_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    do_op(b,c) - do_op(b,c)
}

fn mul_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    do_op(b,c) * do_op(b,c)
}

fn div_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    do_op(b,c) / do_op(b,c)
}

fn not_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    !do_op(b,c)
}

fn and_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    LoxType::Boolean(
        do_op(b,c)
            .into()
        &&
        do_op(b,c)
            .into()
    )
}

fn or_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    LoxType::Boolean(
        do_op(b,c)
            .into()
        ||
        do_op(b,c)
            .into()
    )
}

fn equals_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    LoxType::Boolean(
        do_op(b,c)
        ==
        do_op(b,c)
    )

}

fn not_equals_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    LoxType::Boolean(
        do_op(b,c)
        !=
        do_op(b,c)
    )
}

fn greater_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    LoxType::Boolean(
        do_op(b,c)
        >
        do_op(b,c)
    )
}

fn greater_equal_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    LoxType::Boolean(
        do_op(b,c)
        >=
        do_op(b,c)
    )
}

fn lesser_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    LoxType::Boolean(
        do_op(b,c)
        <
        do_op(b,c)
    )
}

fn lesser_equal_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    LoxType::Boolean(
        do_op(b,c)
        <=
        do_op(b,c)
    )
}

fn var_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    let _idx = read_u8!(b);
    unsafe { stack.push(Var::new(_idx,LoxType::Nil)) };
    LoxType::Nil
}

fn assign_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    let _idx = read_u8!(b);
    unsafe { stack[stack_lookup(_idx).unwrap()].v = do_op(b,c); }
    LoxType::Nil
}

fn pop_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    let amt = read_u8!(b);
    unsafe { stack.truncate(stack.len() - amt as usize) };
    LoxType::Nil
}

fn get_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    let _idx = read_u8!(b);
    unsafe { (&stack[stack_lookup(_idx).unwrap()].v).into_owned() }
}

fn jump_if_true_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    if bool::from(do_op(b,c)) {
        unsafe { idx += read_u8!(b) as usize }
    }
    LoxType::Nil
}

fn jump_if_false_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    if !bool::from(do_op(b,c)) {
        unsafe { idx += read_u8!(b) as usize }
    }
    LoxType::Nil
}

fn jump_back_if_true_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    if bool::from(do_op(b,c)) {
        unsafe { idx -= read_u8!(b) as usize }
    }
    LoxType::Nil
}

fn jump_back_if_false_op(b: &Vec<u8>,c: &Vec<LoxType>) -> VmRes {
    if !bool::from(do_op(b,c)) {
        unsafe { idx -= read_u8!(b) as usize }
    }
    LoxType::Nil
}
