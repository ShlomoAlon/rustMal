use std::cell::RefCell;
use std::collections::HashMap;
use crate::{InnerEnv, MalType};
use crate::MalType::{Bool, List, Num, PrFunc, Str};
use crate::types::MalList;
use anyhow::{anyhow, bail, Context, Error, Result};
pub fn default_env() -> InnerEnv {
    InnerEnv {
        outer: None,
        data: RefCell::new(HashMap::from([
            ("+".to_string(), PrFunc(plus)),
            ("-".to_string(), PrFunc(minus)),
            ("*".to_string(), PrFunc(times)),
            ("/".to_string(), PrFunc(divide)),
            ("list".to_string(), PrFunc(list)),
            ("list?".to_string(), PrFunc(is_list)),
            ("empty?".to_string(), PrFunc(is_empty)),
            ("count".to_string(), PrFunc(count))
        ]))
    }
}

fn plus(x: MalList) -> Result<MalType>{
    let mut result = 0.0;
    for i in x{
        result += i.to_num()?
    }
    Ok(Num(result))
}

fn minus(x: MalList) -> Result<MalType>{
    let mut result = 0.0;
    for i in x{
        result += i.to_num()?
    }
    Ok(Num(result))
}

fn times(x: MalList) -> Result<MalType>{
    let mut result = 1.0;
    for i in x{
        result *= i.to_num()?
    }
    Ok(Num(result))
}

fn divide(x: MalList) -> Result<MalType>{
    let mut x = x.into_iter();
    let mut result = x.next().ok_or_else(|| anyhow!("no values provided to divide"))?.to_num()?;
    for i in x{
        result /= i.to_num()?
    }
    Ok(Num(result))
}
fn list(x: MalList) -> Result<MalType>{
    Ok(List(x))
}
fn is_list(x: MalList) -> Result<MalType>{
    match x.get(0).ok_or_else(|| anyhow!("no items to check"))?{
        List(_) => Ok(Bool(true)),
        _ => Ok(Bool(false))
    }
}
fn is_empty(x: MalList) -> Result<MalType>{
    Ok(Bool(x.get(0).ok_or_else(|| anyhow!("not enough items"))?.clone().to_list()?.len() == 0))
}

fn count(x: MalList) -> Result<MalType>{
    Ok(Num(x.get(0).ok_or_else(|| anyhow!("not enough items"))?.clone().to_list()?.len() as f64))
}
