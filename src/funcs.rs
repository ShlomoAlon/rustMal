use anyhow::{anyhow, bail, Context, Result};
use crate::{eval, MalType, Env, pr_str};
use crate::env::Environment;
use crate::types::MalList;

pub type PrimitiveFuncs = fn(MalList) -> Result<MalType>;

// pub enum MalFunc{
//     PrFunc(PrimitiveFuncs),
//     NFuncs(Func),
// }


#[derive(Debug, Clone)]
pub struct Func {
    parameters: Vec<String>,
    body: MalType,
    environment: Env,
}

impl Func {
    pub fn run_func( &self , values: MalList) -> Result<MalType>{
        let new_env = self.environment.new_env_with_binds(self.parameters.clone(), values);
        eval(self.body.clone(), & new_env?)
    }
    pub fn new(parameters: MalList, body: MalType, environment: Env) -> Result<Self>{
        let mut params = vec![];
        for p in parameters{
            params.push(p.to_symbol()?.to_owned())
        }
        Ok(Func{
            parameters: params,
            body: body,
            environment: environment
        })
    }
    pub fn str_rep(& self) -> String{
        pr_str(self.body.clone())
}

}
