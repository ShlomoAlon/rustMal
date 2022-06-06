use anyhow::{anyhow, bail, Context, Result};

use crate::{MalType, pr_str};
use crate::env::Env;
use crate::env::Environment;
use crate::funcs::Func;
use crate::MalType::{Funcs, List, Nil, PrFunc, Symbol};
use crate::types::{MalIter, MalList};

pub fn eval(ast: MalType, e: &Env) -> Result<MalType> {
    println!(" eval {}", ast);

    match ast {
        MalType::List(mut l) => {
            if l.is_empty() {
                Ok(MalType::List(l))
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
                    _other => eval_func(l, e)
                }
            }
        }
        Symbol(x) => e.get(x),
        other => Ok(other)
    }
}

fn eval_func(lis: MalList, e: &Env) -> Result<MalType> {
    let lis = eval_whole_list(lis, e)?;
    let mut s = lis.into_iter();
    match s.next().unwrap() {
        PrFunc(f) => Ok(f(s.collect())?),
        Funcs(f) => Ok(f.run_func(s.collect())?),
        other => bail!("{} not a valid function", other)
    }
}

fn eval_whole_list(lis: MalList, e: &Env) -> Result<MalList> {
    lis.into_iter().map(|x| eval(x, e)).collect()
}


fn eval_def(lis: MalList, e: &Env) -> Result<MalType> {
    let h = eval(lis.get(2).unwrap().clone(), e)?;
    e.set(lis.get(1).unwrap().to_symbol().unwrap().clone(), h.clone());
    Ok(h)
}

fn create_bindings_for_let_block(lis: MalList, e: &Env) -> Result<()> {
    let mut i = lis.into_iter().peekable();
    while i.peek().is_some() {
        let symbol = i.next().unwrap().to_symbol()?.to_string();
        let value = eval(i.next().ok_or(anyhow!("to many symbols"))?, e);
        e.set(symbol, value.unwrap());
    }
    Ok(())
}

fn eval_let(lis: MalList, e: &Env) -> Result<MalType> {
    let new_env = e.new_env();
    let mut lis = lis.into_iter();
    lis.next();
    let first_block = lis.next().unwrap().to_list()?;
    create_bindings_for_let_block(first_block, &new_env)?;
    eval(lis.next().unwrap(), &new_env)
}

fn eval_if(lis: MalList, e: &Env) -> Result<MalType> {
    let mut lis = lis.into_iter();
    lis.next();
    if eval(lis.next().unwrap(), e)?.not_nil_or_false() {
        eval(lis.next().unwrap(), e)
    } else {
        lis.next();
        eval(lis.next().unwrap_or(Nil), e)
    }
}

fn eval_do(lis: MalList, e: &Env) -> Result<MalType> {
    let mut lis = lis.into_iter();
    lis.next();
    lis.map(|i| eval(i, e)).last().unwrap()
}

fn eval_fn(l: MalList, e: &Env) -> Result<MalType> {
    let mut i = l.into_iter();
    i.next();
    Ok(MalType::Funcs(Box::new(Func::new(i.next().unwrap().to_list().unwrap(),
                                         i.next().unwrap(),
                                         e.clone()).unwrap())))
}


#[cfg(test)]
mod tests {
    use crate::{pr_str, read};
    use crate::printer;

    use super::*;

    #[test]
    fn test_read() {
        let x = read("(+ 1 1)".to_string());
        assert_eq!(pr_str(x.unwrap()), "((+ 1 1))".to_string())
    }
}
