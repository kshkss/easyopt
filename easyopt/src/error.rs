use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed with apply a function due to invalid variables")]
    InvalidVariable,
    #[error("a condition is violated")]
    ConditionViolated,
    #[error("an error occurs: {0}")]
    Failure(String),
}
