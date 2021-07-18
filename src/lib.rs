mod dual;
mod traits;

use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("failed with apply a function due to invalid variables")]
    InvalidVariable,
    #[error("a condition is violated")]
    ConditionViolated,
}

pub use dual::{Dual, Variables};
mod self_consistent;

use crate::traits::*;

pub struct Executor<S, O, T> {
    solver: S,
    op: O,
    criteria: Option<T>,
}

impl<S, O, T> Executor<S, O, T>
where
    S: Solver<O>,
    O: Op,
    T: Criteria<Variable = O::Variable>,
{
    pub fn new(solver: S, op: O) -> Self {
        Self {
            solver: solver,
            op: op,
            criteria: None,
        }
    }

    pub fn run(&mut self, init: O::Variable) -> Result<O::Variable, Error> {
        let mut x = init;
        let mut x_new = self.solver.next_iter(&self.op, &x)?;
        let mut res = self.criteria.as_ref().unwrap().apply(&x_new, &x);
        while res.is_err() {
            x = x_new;
            x_new = self.solver.next_iter(&self.op, &x)?;
            res = self.criteria.as_ref().unwrap().apply(&x_new, &x);
        }
        Ok(x_new)
    }

    pub fn terminate(self, c: T) -> Self {
        Self {
            criteria: Some(c),
            ..self
        }
    }
}

pub struct Tolerance {
    e: f64,
}

impl Tolerance {
    pub fn new(e: f64) -> Self {
        Self { e: e }
    }
}

impl Criteria for Tolerance {
    type Variable = f64;
    fn apply(&self, xnew: &f64, x: &f64) -> Result<(), f64> {
        let diff = xnew - x;
        if diff.is_nan() {
            panic!();
        }
        if diff.abs() > self.e {
            Err(diff)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
