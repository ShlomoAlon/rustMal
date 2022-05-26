use std::collections::HashMap;
use std::fmt;
use std::fmt::Error;

pub type MalList = Vec<MalType>;
pub type MalIter = std::vec::IntoIter<MalType>;
use enum_as_inner::EnumAsInner;
use lazy_static::lazy_static;
use crate::MalType::{Bool, PrFunc, List, Num, Str, Symbol};
use crate::{Env, eval, pr_str, RcEnv};
use crate::env::Environment;
use crate::reader::BoxResult;

#[derive(Debug, EnumAsInner, Clone)]
pub enum MalType{
    Nil,
    Bool(bool),
    Str(String),
    Symbol(String),
    List(MalList),
    Num(f64),
    PrFunc(PrimitiveFuncs),
    Func(Box<Func>)
}
impl fmt::Display for MalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", pr_str(self.clone()))
    }
}

impl MalType  {
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
    pub fn to_func(&self) -> Option<&PrimitiveFuncs> {
        match self {
            PrFunc(x) => Some(x),
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
pub type PrimitiveFuncs = fn(MalList) -> MalType;


#[derive(Debug, Clone)]
pub struct Func {
    pub(crate) parameters: MalList,
    pub(crate) body: MalType,
    pub(crate) environment: RcEnv,
}

impl Func {
    pub fn run_func( &self , values: MalList) -> MalType{
        let new_env = self.environment.new_env_with_binds(self.parameters.clone(), values);
        eval(self.body.clone(), & new_env)

    }

}













