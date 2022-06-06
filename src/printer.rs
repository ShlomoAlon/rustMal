use crate::MalType;

pub fn pr_str(ast: MalType) -> String{
    match ast {
        MalType::Nil=> "".to_string(),
        MalType::Bool(x) => x.to_string(),
        MalType::Str(x) => x,
        MalType::Symbol(x) => x,
        MalType::List(lis) => {
            "(".to_owned() +  &lis.into_iter().
                map(|x| pr_str(x)).
                collect::<Vec<String>>().
                join(" ") + ")"}
        MalType::Num(x) => {x.to_string()}
        MalType::PrFunc(_) => "func".to_string(),
        MalType::Funcs(x) => {
            x.str_rep()
        }
    }
}