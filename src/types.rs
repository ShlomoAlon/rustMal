use std::fmt;

use anyhow::{anyhow, bail, Context, Result};
use enum_as_inner::EnumAsInner;

use crate::{InnerEnv, eval, pr_str, Env};
use crate::env::Environment;
use crate::funcs::{Func, PrimitiveFuncs};
use crate::MalType::{Bool, List, Num, PrFunc, Str, Symbol};

pub type MalList = Vec<MalType>;
pub type MalIter = std::vec::IntoIter<MalType>;

#[derive(Debug, EnumAsInner, Clone)]
pub enum MalType {
    Nil,
    Bool(bool),
    Str(String),
    Symbol(String),
    List(MalList),
    Num(f64),
    PrFunc(PrimitiveFuncs),
    Funcs(Box<Func>),
}

impl fmt::Display for MalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", pr_str(self.clone()))
    }
}

impl MalType {
    pub fn to_list(self) -> Result<MalList> {
        match self {
            List(x) => Ok(x),
            other => Err(anyhow!("{} is not a list", other))
        }
    }
    pub fn to_num(&self) -> Result<f64> {
        match self {
            Num(x) => Ok(*x),
            other => Err(anyhow!("{} is Not a num", other))
        }
    }
    pub fn to_bool(self) -> Result<bool> {
        match self {
            Bool(x) => Ok(x),
            other => Err(anyhow!("{} is Not a bool", other))
        }
    }

    pub fn to_str(&self) -> Result<&String> {
        match self {
            Str(x) => Ok(x),
            other => Err(anyhow!("{} is not a string", other))
        }
    }
    pub fn to_func(&self) -> Result<&PrimitiveFuncs> {
        match self {
            PrFunc(x) => Ok(x),
            other => Err(anyhow!("{} is not a func", other))
        }
    }
    pub fn to_symbol(&self) -> Result<&String> {
        match self {
            Symbol(x) => Ok(x),
            other => Err(anyhow!("{} not a symbol", other))
        }
    }
    pub fn not_nil_or_false(&self) -> bool {
        match self {
            MalType::Nil => false,
            Bool(bool) => *bool,
            _ => true
        }
    }
}














