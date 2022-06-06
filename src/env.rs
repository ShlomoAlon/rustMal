use std::borrow::Borrow;
use anyhow::{anyhow, bail, Context, Result};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::MalType;
use crate::types::MalList;

pub type MalEnv = HashMap<String, MalType>;
#[derive(Debug)]
pub struct InnerEnv {
    pub outer:  Option<Env>,
    pub data: RefCell<MalEnv>
}
pub type Env = Rc<InnerEnv>;

pub(crate) trait Environment{
    fn set(&self, symbol: String, m: MalType);
    fn find(&self, symbol: String) -> Result<MalType>;
    fn get(&self, symbol: String) -> Result<MalType>;
    fn new_env(&self) -> Env;
    fn new_env_with_binds(&self, binds: Vec<String>, exprs: MalList) -> Result<Env>;
}
impl Environment for Env {
    fn set(&self, symbol: String, m: MalType) {
        self.data.borrow_mut().insert(symbol, m);
    }
    fn find(&self, symbol: String) -> Result<MalType>{
        match self.data.borrow().get(&symbol) {
            Some(t) => Ok(t.clone()),
            None => match self.outer.borrow() {
                Some(o) => o.find(symbol),
                None => Err(anyhow!("no symbol matching {} in environment", symbol))
            }
        }
    }
    fn get(&self, symbol: String) -> Result<MalType> {
        Ok(self.find(symbol)?)
    }
    fn new_env(&self) -> Env {
        Rc::new(InnerEnv {
            outer: Some(self.clone()),
            data: RefCell::new(HashMap::new())
        })
    }
    fn new_env_with_binds(&self, binds: Vec<String>, exprs: MalList) -> Result<Env> {
        let result = self.new_env();
        let mut s1 = binds.into_iter().peekable();
        let mut s2 = exprs.into_iter().peekable();
        while s1.peek().is_some() || s2.peek().is_some() {
            result.set(s1.next().ok_or_else(|| anyhow!("to few parameters"))?,
                       s2.next().ok_or_else(|| anyhow!("to many parameters"))?)
        }
        Ok(result)
    }
}
