use super::SelfConsistentOp;
use crate::error::*;
use crate::traits::*;
use num_traits::One;
//use std::ops::{Add, Div, Mul, Sub};

pub struct Wegstein<T> {
    y_prev: Option<T>,
    x_prev: Option<T>,
}

impl<T> Wegstein<T> {
    pub fn new() -> Self {
        Self {
            y_prev: None,
            x_prev: None,
        }
    }
}

impl<T, U> Solver<T> for Wegstein<U>
where
    T: SelfConsistentOp<Variable = U>,
    T::Scalar: One + BinaryOperand<U, U> + for<'a> BinaryOperand<&'a U, U>,
    for<'a> U: Clone + BinaryOperand<&'a U, U>,
    for<'a, 'b> &'a U: BinaryOperand<&'b U, U>,
{
    type Variable = T::Variable;
    type ReportArg = T::Variable;
    fn next_iter(&mut self, op: &T, x: &T::Variable) -> Result<T::Variable, Error> {
        let y = op.apply(&x)?;
        if let Some(y_prev) = self.y_prev.as_ref() {
            let x_prev = self.x_prev.as_ref().unwrap();
            let s = (&y - &y_prev) / &(x - &x_prev);
            let t = T::Scalar::one() / (T::Scalar::one() - &s);
            let next = (&t * &y) + &((T::Scalar::one() - &t) * &x);
            self.x_prev.replace(x.clone());
            self.y_prev.replace(y);
            Ok(next)
        } else {
            self.x_prev.replace(x.clone());
            self.y_prev.replace(y.clone());
            Ok(y)
        }
    }
    fn init_report<R: Report<Arg = T::Variable, Op = T>>(
        &self,
        report: &mut R,
        op: &T,
        x: &T::Variable,
    ) -> Result<(), Error> {
        report.init(op, x)
    }

    fn update_report<R: Report<Arg = T::Variable, Op = T>>(
        &self,
        report: &mut R,
        op: &T,
        x: &T::Variable,
    ) -> Result<(), Error> {
        report.update(op, x)
    }
}

pub struct Steffensen;

impl Steffensen {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T> Solver<T> for Steffensen
where
    T: SelfConsistentOp,
    for<'a> T::Variable: BinaryOperand<&'a T::Variable, T::Variable>,
    for<'a, 'b> &'a T::Variable: BinaryOperand<&'b T::Variable, T::Variable>,
{
    type Variable = T::Variable;
    type ReportArg = T::Variable;
    fn next_iter(&mut self, op: &T, x: &T::Variable) -> Result<T::Variable, Error> {
        let y = op.apply(&x)?;
        let z = op.apply(&y)? - &y;
        let y = y - &x;
        let x = x - &(&y * &y / &(z - &y));
        Ok(x)
    }
    fn init_report<R: Report<Arg = T::Variable, Op = T>>(
        &self,
        report: &mut R,
        op: &T,
        x: &T::Variable,
    ) -> Result<(), Error> {
        report.init(op, x)
    }

    fn update_report<R: Report<Arg = T::Variable, Op = T>>(
        &self,
        report: &mut R,
        op: &T,
        x: &T::Variable,
    ) -> Result<(), Error> {
        report.update(op, x)
    }
}
