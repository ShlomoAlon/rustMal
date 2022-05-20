use std::array::IntoIter;
use std::fmt::Error;
use crate:: {MalType, pr_str};
use crate::env::{default_env, Env};
use crate::MalType::{List, Nil, Symbol};
use crate::types::{MalFunc, MalIter, MalList};
use crate::reader::BoxResult;

pub fn eval(ast: MalType, e: & mut Env) -> MalType{
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
                    other => eval_func(l, e)
                }
            }

        }
        Symbol(x) => e.get(x).unwrap().clone(),
        other => other
    }
}
fn eval_func(lis: MalList,e :& mut Env) -> MalType{
    let lis = eval_whole_list(lis, e);
    let mut s = lis.into_iter();
    let f = *s.next().unwrap().
        to_func().unwrap();
    f(s.collect())
}

fn eval_whole_list(lis: MalList, e: & mut Env) -> MalList{
    lis.into_iter().map(|x| eval(x, e)).collect()
}


fn eval_def(lis: MalList, e: &mut Env) -> MalType{
    let h = eval(lis.get(2).unwrap().clone(), e);
    e.set(lis.get(1).unwrap().to_symbol().unwrap().clone(),h.clone());
    println!("{:#?}", e);
    h
}

fn create_bindings_for_let_block(lis: MalList, e: &mut Env){
    let mut i = lis.into_iter().peekable();
    while i.peek().is_some() {
        let symbol = i.next().unwrap().to_symbol().unwrap().to_string();
        let value = eval(i.next().unwrap(), e);
        e.set(symbol, value);
    }
}
fn eval_let(lis: MalList, e: &mut Env) -> MalType{
    let mut new_env = e.new_env();
    let mut lis = lis.into_iter();
    lis.next();
    let first_block = lis.next().unwrap().to_list().unwrap();
    create_bindings_for_let_block(first_block, & mut new_env);
    eval(lis.next().unwrap(), & mut new_env)
}
fn eval_if(lis: MalList, e: &mut Env) -> MalType{
    let mut lis = lis.into_iter();
    lis.next();
    if eval(lis.next().unwrap(), e).not_nil_or_false(){
        eval(lis.next().unwrap(), e)
    } else {
        lis.next();
        eval(lis.next().unwrap_or(Nil), e)
    }
}
// fn eval_do(lis: MalList, e: &mut Env) -> MalType{
//     let mut lis = lis.into_iter();
//     lis.next();
//     lis.map(|i| eval_ast(i, e)).last().unwrap().unwrap()
// }

// fn eval_symbol


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
