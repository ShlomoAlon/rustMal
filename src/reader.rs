use lazy_static::lazy_static;
use regex::{Regex};
use crate::types::{MalList, MalType};
use core::fmt::Error;
type Token = String;
pub type BoxResult<T> = Result<T,Box<Error>>;
lazy_static! {
        static ref RE: Regex = Regex::new(
        r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"###
        ).unwrap();}
#[derive(Debug)]
struct Reader {
    tokens:Vec<Token>,
    position: usize,
}

impl Reader {
    fn next(& mut self) {
        self.position += 1
    }

    fn peek(&self) -> BoxResult<Token>{
        Ok(self.tokens.get(self.position).ok_or(Error)?.to_owned())
    }
}

fn tokenize(text: &str) -> Vec<Token>{
    let mut result: Vec<Token> = Vec::new();
    for cap in RE.captures_iter(& text){
        result.push(cap[1].to_string())
    }
    result

}

pub fn read_str(text: &str) -> BoxResult<MalType>{
    let mut r = Reader{
        tokens: tokenize(text),
        position: 0,};
    // println!(" read str {:?}", r);
    match r.peek() {
        Ok(token) => match token.as_str() {
            "(" => read_form(&mut r),
             _ => {
                 r.tokens.pop();
                 r.tokens.insert(0, "(".to_string());
                 r.tokens.push(")".to_string());
                 read_list(& mut r)}},
        Err(_) => Ok(MalType::List(vec![]))
    }
}
fn read_form(r: & mut Reader) -> BoxResult<MalType> {
    // println!("read form {:?}", r);
    match r.peek().unwrap().as_str() {
        "(" => read_list(r),
         _ => read_atom(r)
    }
}
fn read_list(r: &mut Reader) -> BoxResult<MalType> {
    // println!(" read list {:?}", r);
    let mut v: MalList = vec![];
    r.next();
    loop {
        let x = r.peek()?;
        if x == ")" {
            r.next();
            break
        } else {
            v.push(read_form(r)?)
        }
    }
    Ok(MalType::List(v))
}
fn read_atom(r: &mut Reader) -> BoxResult<MalType> {
    // println!(" read atom {:?}", r);
    let x = r.peek().unwrap();
    r.next();
    if x.chars().nth(0).unwrap() == '"' {
        Ok(MalType::Str(x))
    } else if x.chars().nth(0).unwrap().is_numeric() {
        Ok(MalType::Num(x.parse().unwrap()))
    } else if x == "true" {
        Ok(MalType::Bool(true))
    } else if x == "false" {
        Ok(MalType::Bool(false))
    } else {
        Ok(MalType::Symbol(x))
    }

}
