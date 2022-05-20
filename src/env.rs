use std::collections::HashMap;
use std::fmt::Error;
use std::rc::Rc;
use crate::MalType;
use crate::MalType::{Func, Num};
use crate::reader::BoxResult;
use crate::types::{MalFunc, MalList};

pub type MalEnv = HashMap<String, MalType>;
#[derive(Debug)]
pub struct Env<'a>{
    outer:  Box<Option<&'a Env<'a>>>,
    pub(crate) data: MalEnv
}



impl  Env<'_>{
    pub(crate) fn set(&mut self, symbol: String, m: MalType){
        self.data.insert(symbol, m);
    }
    fn find(&self, symbol: String) -> Option<&MalType> {
        match self.data.get(&symbol) {
            Some(t) => Some(t),
            None => match *self.outer {
                Some(o) => o.find(symbol),
                None => None
            }
        }
    }
    pub(crate) fn get(&self, symbol: String) -> BoxResult<&MalType> {
        Ok(self.find(symbol).ok_or(Error)?)

    }
    pub fn new_env(&self) -> Env{
        Env {
            outer: Box::new(Some(self)),
            data: HashMap::new()
        }
    }



}
pub fn default_env() -> Env<'static>{
    Env {
        outer: Box::new(None),
        data: HashMap::from([
            ("+".to_string(), Func(plus)),
            ("-".to_string(), Func(minus)),
            ("*".to_string(), Func(times)),
            ("/".to_string(), Func(divide)),
        ])
    }
}
fn plus(x: MalList) -> MalType{
    let mut result = 0.0;
    for i in x{
        result += match i {
            Num(z) => z,
            other => 0.0
        }
    }
    MalType::Num(result)
}
fn minus(x: MalList) -> MalType{
    let mut result = 0.0;
    for i in x{
        result -= match i {
            MalType::Num(z) => z,
            _ => 0.0
        }
    }
    MalType::Num(result)
}
fn times(x: MalList) -> MalType{
    let mut result = 1.0;
    for i in x{
        result *= match i {
            MalType::Num(z) => z,
            _ => 1.0
        }
    }
    MalType::Num(result)
}
fn divide(x: MalList) -> MalType{
    let mut x = x.into_iter();
    let mut result = x.next().unwrap().to_num().unwrap();
    for i in x{
        result /= i.to_num().unwrap()
    }
    Num(result)}
