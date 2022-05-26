use std::array::IntoIter;
use std::fmt::Error;
use std::fs::copy;
use crate:: {MalType, pr_str};
use crate::core::default_env;
use crate::env::{Env, RcEnv};
use crate::MalType::{List, Nil, PrFunc, Symbol};
use crate::types::{Func, MalIter, MalList, PrimitiveFuncs};
use crate::reader::BoxResult;
use crate::env::Environment;


pub fn eval(ast: MalType, e: & RcEnv) -> MalType{
    println!(" eval {}" , ast);
    
    match ast {
        MalType::List(mut l) => {
            let length = l.len();
            if l.len() == 0{
                MalType::List(l)
            } else if l.len() == 1 {
                eval(l.pop().unwrap(), e)
            } else {
                let x = l.get(0).unwrap();
                match x.to_symbol().unwrap_or(&"".to_string()).as_str() {
                    "def!" => eval_def(l, e),
                    "let*" => eval_let(l, e),
                    "if" => eval_if(l, e),
                    "do" => eval_do(l, e),
                    "fn*" => eval_fn(l, e),
                    other => eval_func(l, e)
                }
            }

        }
        Symbol(x) => e.get(x).unwrap().clone(),
        other => other
    }
}
fn eval_func(lis: MalList,e :& RcEnv) -> MalType{
    let lis = eval_whole_list(lis, e);
    let mut s = lis.into_iter();
    match s.next().unwrap() {
        PrFunc(f) => f(s.collect()),
        MalType::Func(f) => f.run_func(s.collect()),
        other => panic!()
    }
}

fn eval_whole_list(lis: MalList, e: & RcEnv) -> MalList{
    lis.into_iter().map(|x| eval(x, e)).collect()
}


fn eval_def(lis: MalList, e: & RcEnv) -> MalType{
    let h = eval(lis.get(2).unwrap().clone(), e);
    e.set(lis.get(1).unwrap().to_symbol().unwrap().clone(),h.clone());
    println!("{:#?}", e);
    h
}

fn create_bindings_for_let_block(lis: MalList, e: & RcEnv){
    let mut i = lis.into_iter().peekable();
    while i.peek().is_some() {
        let symbol = i.next().unwrap().to_symbol().unwrap().to_string();
        let value = eval(i.next().unwrap(), e);
        e.set(symbol, value);
    }
}
fn eval_let(lis: MalList, e: & RcEnv) -> MalType{
    let new_env = e.new_env();
    let mut lis = lis.into_iter();
    lis.next();
    let first_block = lis.next().unwrap().to_list().unwrap();
    create_bindings_for_let_block(first_block, & new_env);
    eval(lis.next().unwrap(), & new_env)
}
fn eval_if(lis: MalList, e: & RcEnv) -> MalType{
    let mut lis = lis.into_iter();
    lis.next();
    if eval(lis.next().unwrap(), e).not_nil_or_false(){
        eval(lis.next().unwrap(), e)
    } else {
        lis.next();
        eval(lis.next().unwrap_or(Nil), e)
    }
}
fn eval_do(lis: MalList, e: & RcEnv) -> MalType{
    let mut lis = lis.into_iter();
    lis.next();
    lis.map(|i| eval(i, e)).last().unwrap()
}

fn eval_fn(l: MalList, e: & RcEnv) -> MalType{
    let mut i = l.into_iter();
    i.next();
    MalType::Func(Box::from(Func {
        parameters: i.next().unwrap().to_list().unwrap(),
        body: i.next().unwrap(),
        environment: e.clone()
    }))
}





#[cfg(test)]
mod tests {
    use crate::{pr_str, read};
    use crate::printer;
    use super::*;

    #[test]
    fn test_read(){
        let x = read("(+ 1 1)".to_string());
        assert_eq!(pr_str(x), "(+ 1 1)".to_string())
    }

}
