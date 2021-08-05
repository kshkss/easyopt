use super::SelfConsistentOp;
use crate::error::*;
use crate::traits::*;
use num_traits::One;
use std::marker::PhantomData;
//use std::ops::{Add, Div, Mul, Sub};

pub struct Wegstein<T, K = f64> {
    y_prev: Option<T>,
    x_prev: Option<T>,
    _scalar_type: PhantomData<K>,
}

impl<T, F> Wegstein<T, F> {
    pub fn new() -> Self {
        Self {
            y_prev: None,
            x_prev: None,
            _scalar_type: PhantomData,
        }
    }
}

impl<T, U, F> Solver<T> for Wegstein<U, F>
where
    T: SelfConsistentOp<Variable = U>,
    F: One + BinaryOperand<U, U> + for<'a> BinaryOperand<&'a U, U>,
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
            let t = F::one() / (F::one() - &s);
            let next = (&t * &y) + &((F::one() - &t) * &x);
            self.x_prev.replace(x.clone());
            self.y_prev.replace(y);
            Ok(next)
        } else {
            self.x_prev.replace(x.clone());
            self.y_prev.replace(y.clone());
            Ok(y)
        }
    }
    fn init_report<R: Report<Arg = T::Variable>>(
        &self,
        report: &mut R,
        x: &T::Variable,
    ) -> Result<(), Error> {
        report.init(x)
    }

    fn update_report<R: Report<Arg = T::Variable>>(
        &self,
        report: &mut R,
        x: &T::Variable,
    ) -> Result<(), Error> {
        report.update(x)
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
    fn init_report<R: Report<Arg = T::Variable>>(
        &self,
        report: &mut R,
        x: &T::Variable,
    ) -> Result<(), Error> {
        report.init(x)
    }

    fn update_report<R: Report<Arg = T::Variable>>(
        &self,
        report: &mut R,
        x: &T::Variable,
    ) -> Result<(), Error> {
        report.update(x)
    }
}
