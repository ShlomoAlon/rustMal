use thiserror::Error;
#[derive(Error, Debug)]
pub enum MalErr{
    #[error("{symbol}")]
    InvalidSymbol{
        symbol: String
    }
}