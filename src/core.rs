use std::cell::RefCell;
use std::collections::HashMap;
use crate::{Env, MalType};
use crate::MalType::{Num, PrFunc};
use crate::types::MalList;

pub fn default_env() -> Env{
    Env {
        outer: None,
        data: RefCell::new(HashMap::from([
            ("+".to_string(), PrFunc(plus)),
            ("-".to_string(), PrFunc(minus)),
            ("*".to_string(), PrFunc(times)),
            ("/".to_string(), PrFunc(divide)),
        ]))
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
