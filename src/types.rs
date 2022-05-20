use std::collections::HashMap;
use std::fmt;
use std::fmt::Error;

pub type MalList = Vec<MalType>;
pub type MalIter = std::vec::IntoIter<MalType>;
use enum_as_inner::EnumAsInner;
use lazy_static::lazy_static;
use crate::MalType::{Bool, Func, List, Num, Str, Symbol};
use crate::pr_str;
use crate::reader::BoxResult;

#[derive(Debug, EnumAsInner, Clone)]
pub enum MalType{
    Nil,
    Bool(bool),
    Str(String),
    Symbol(String),
    List(MalList),
    Num(f64),
    Func(MalFunc)
}
impl fmt::Display for MalType {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", pr_str(self.clone()))
    }
}

impl MalType {
    pub fn to_list(self) -> Option<MalList> {
        match self {
            List(x) => Some(x),
            other => None
        }
    }
    pub fn to_num(& self) -> Option<f64> {
        match self {
            Num(x) => Some(*x),
            other => None
        }
    }
    pub fn to_bool(self) -> Option<bool> {
        match self {
            Bool(x) => Some(x),
            other => None
        }
    }

    pub fn to_str(&self) -> Option<&String> {
        match self {
            Str(x) => Some(x),
            other => None
        }
    }
    pub fn to_func(&self) -> Option<&MalFunc> {
        match self {
            Func(x) => Some(x),
            other => None
        }
    }
    pub fn to_symbol(&self) -> Option<&String> {
        match self {
            Symbol(x) => Some(x),
            other => None
        }
    }
    pub fn not_nil_or_false(&self) -> bool{
        match self {
            MalType::Nil => false,
            Bool(bool) => *bool,
            other=> true

        }
    }

}
pub struct  MalError{

}

type MalRet = Result<MalType, MalError>;
pub type MalFunc = fn(MalList) -> MalType;













