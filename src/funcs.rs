use anyhow::{anyhow, bail, Context, Result};
use crate::{eval, MalType, RcEnv};
use crate::env::Environment;
use crate::types::MalList;

pub type PrimitiveFuncs = fn(MalList) -> MalType;

// pub enum MalFunc{
//     PrFunc(PrimitiveFuncs),
//     NFuncs(Func),
// }



#[derive(Debug, Clone)]
pub struct Func {
    pub(crate) parameters: Vec<String>,
    pub(crate) body: MalType,
    pub(crate) environment: RcEnv,
}

impl Func {
    pub fn run_func( &self , values: MalList) -> MalType{
        let new_env = self.environment.new_env_with_binds(self.parameters.clone(), values);
        eval(self.body.clone(), & new_env.unwrap())
    }
    pub fn new(parameters: MalList, body: MalType, environment: RcEnv) -> Result<Self>{
        let paramter
        Ok(Func{
            parameters:
            body: body,
            environment: environment
        })
    }

}
