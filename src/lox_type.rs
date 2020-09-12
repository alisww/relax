use super::err::LoxError;
//use super::{Expr,Callable};
use std::fmt;
use std::convert::TryFrom;
use std::cmp::{Ordering,PartialOrd,PartialEq};
use std::boxed::Box;
use std::ops::{Sub,Add,Mul,Div,Not};

#[derive(Debug,Clone)]
pub enum LoxType {
    String(String),
    Number(f64),
    Nil,
    Boolean(bool),
    //Callable(Box<Callable>)
}

impl PartialEq for LoxType {
    fn eq(&self,other: &LoxType) -> bool {
        match (self,other) {
            (&LoxType::String(ref s),&LoxType::String(ref o)) => (s == o),
            (&LoxType::Number(ref s),&LoxType::Number(ref o)) => (s == o),
            (&LoxType::Nil,&LoxType::Nil) => true,
            (&LoxType::Boolean(ref s),&LoxType::Boolean(ref o)) => (s == o),
            _ => false
        }
    }
}

impl PartialOrd for LoxType {
    fn partial_cmp(&self,other: &LoxType) -> Option<Ordering> {
        match (self,other) {
            (&LoxType::String(ref s),&LoxType::String(ref o)) => (s.partial_cmp(o)),
            (&LoxType::Number(ref s),&LoxType::Number(ref o)) => (s.partial_cmp(o)),
            (&LoxType::Nil,&LoxType::Nil) => Some(Ordering::Equal),
            (&LoxType::Boolean(ref s),&LoxType::Boolean(ref o)) => (s.partial_cmp(o)),
            _ => None
        }
    }
}

impl Add for LoxType {
    type Output = Self;

    fn add(self,other: Self) -> Self {
        match (self,other) {
            (LoxType::String(s),LoxType::String(o)) => LoxType::String(s + &o),
            (LoxType::Number(s),LoxType::Number(o)) => LoxType::Number(s + o),
            (LoxType::Nil,LoxType::Nil) => LoxType::Nil,
            (LoxType::Boolean(s),LoxType::Boolean(o)) => LoxType::Boolean(s || o),
            _ => LoxType::Nil
        }
    }
}

impl Sub for LoxType {
    type Output = Self;

    fn sub(self,other: Self) -> Self {
        match (self,other) {
            (LoxType::String(s),LoxType::String(o)) => LoxType::Nil,
            (LoxType::Number(s),LoxType::Number(o)) => LoxType::Number(s - o),
            (LoxType::Nil,LoxType::Nil) => LoxType::Nil,
            (LoxType::Boolean(s),LoxType::Boolean(o)) => LoxType::Nil,
            _ => LoxType::Nil
        }
    }
}

impl Mul for LoxType {
    type Output = Self;

    fn mul(self,other: Self) -> Self {
        match (self,other) {
            (LoxType::String(s),LoxType::String(o)) => LoxType::Nil,
            (LoxType::Number(s),LoxType::Number(o)) => LoxType::Number(s * o),
            (LoxType::Nil,LoxType::Nil) => LoxType::Nil,
            (LoxType::Boolean(s),LoxType::Boolean(o)) => LoxType::Boolean(s && o),
            _ => LoxType::Nil
        }
    }
}

impl Div for LoxType {
    type Output = Self;

    fn div(self,other: Self) -> Self {
        match (self,other) {
            (LoxType::String(s),LoxType::String(o)) => LoxType::Nil,
            (LoxType::Number(s),LoxType::Number(o)) => LoxType::Number(s / o),
            (LoxType::Nil,LoxType::Nil) => LoxType::Nil,
            (LoxType::Boolean(s),LoxType::Boolean(o)) => LoxType::Nil,
            _ => LoxType::Nil
        }
    }
}

impl Not for LoxType {
    type Output = Self;

    fn not (self) -> Self {
        match self {
            LoxType::String(s)=> LoxType::Nil,
            LoxType::Number(s) => LoxType::Number(-s),
            LoxType::Nil => LoxType::Nil,
            LoxType::Boolean(s) => LoxType::Boolean(!s),
            _ => LoxType::Nil
        }
    }
}

impl fmt::Display for LoxType {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &LoxType::String(ref s) => write!(f,"{}",s),
            &LoxType::Number(ref n) => write!(f,"{}",n),
            &LoxType::Boolean(ref b) => write!(f,"{}",b),
            &LoxType::Nil => write!(f,"nil"),
            _ => Ok(())
        }
    }
}

impl TryFrom<LoxType> for f64 {
    type Error = LoxError;
    fn try_from(value: LoxType) -> Result<Self,Self::Error> {
        if let LoxType::Number(n) = value {
            Ok(n)
        } else {
            Err(LoxError::new("Failed to cast LoxType into f64".to_string(),0))
        }
    }
}

impl TryFrom<LoxType> for String {
    type Error = LoxError;
    fn try_from(value: LoxType) -> Result<Self,Self::Error> {
        if let LoxType::String(s) = value {
            Ok(s)
        } else {
            Err(LoxError::new("Failed to cast LoxType into String".to_string(),0))
        }
    }
}
/***
impl TryFrom<LoxType> for bool {
    type Error = LoxError;
    fn try_from(value: LoxType) -> Result<Self,Self::Error> {
        if let LoxType::Boolean(b) = value {
            Ok(b)
        } else {
            Err(LoxError::new("Failed to cast LoxType into bool".to_string(),0))
        }
    }
}***/

impl From<LoxType> for bool {
    fn from(value: LoxType) -> Self {
        match value {
            LoxType::Nil => false,
            LoxType::Boolean(b) => b,
            _ => true
        }
    }
}

impl TryFrom<LoxType> for () {
    type Error = LoxError;
    fn try_from(value: LoxType) -> Result<Self,Self::Error> {
        if let LoxType::Nil = value {
            Ok(())
        } else {
            Err(LoxError::new("Failed to cast LoxType into ()".to_string(),0))
        }
    }
}
