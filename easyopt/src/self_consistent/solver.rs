use super::{SelfConsistentOp, SelfConsistentOpSolver};
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

impl<T, U, F> SelfConsistentOpSolver<T> for Wegstein<U, F>
where
    T: Op<Variable = U> + SelfConsistentOp<Variable = <T as Op>::Variable>,
    F: One + BinaryOperand<U, U> + for<'a> BinaryOperand<&'a U, U>,
    for<'a> U: Clone + BinaryOperand<&'a U, U>,
    for<'a, 'b> &'a U: BinaryOperand<&'b U, U>,
{
    fn next_iter(&mut self, op: &T, x: &<T as Op>::Variable) -> Result<<T as Op>::Variable, Error> {
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
}

pub struct Steffensen;

impl Steffensen {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T> SelfConsistentOpSolver<T> for Steffensen
where
    T: Op + SelfConsistentOp<Variable = <T as Op>::Variable>,
    for<'a> <T as Op>::Variable: BinaryOperand<&'a <T as Op>::Variable, <T as Op>::Variable>,
    for<'a, 'b> &'a <T as Op>::Variable:
        BinaryOperand<&'b <T as Op>::Variable, <T as Op>::Variable>,
{
    fn next_iter(&mut self, op: &T, x: &<T as Op>::Variable) -> Result<<T as Op>::Variable, Error> {
        let y = op.apply(&x)?;
        let z = op.apply(&y)? - &y;
        let y = y - &x;
        let x = x - &(&y * &y / &(z - &y));
        Ok(x)
    }
}
