use super::SelfConsistentOp;
use crate::traits::{BinaryOperand, Op, Solver};
use crate::Error;
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

impl<T> Solver<T> for Steffensen
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

#[cfg(test)]
mod test {
    use super::*;
    use approx::relative_eq;
    use crate::{Executor, Tolerance};

    struct TestCase01 {
        a: f64,
        b: f64,
        c: f64,
    }
    impl SelfConsistentOp for TestCase01 {
        type Variable = f64;
        fn apply(&self, x: &f64) -> Result<f64, Error> {
            Ok(self.a * x * x + self.b * x + self.c)
        }
    }

    #[test]
    fn case01_wegstein() {
        let op = TestCase01 {
            a: 1.,
            b: 1.,
            c: -2.,
        };
        let solver = Wegstein::<f64>::new();
        let x = Executor::new(solver, op).terminate(Tolerance::new(1e-6)).run(2.).unwrap();
        assert!(relative_eq!(f64::sqrt(2.), x))
    }

    #[test]
    fn case02_wegstein() {
        let op = |x: &f64| -> f64 { x * x + x - 2. };
        let solver = Wegstein::<f64>::new();
        let x = Executor::new(solver, op).terminate(Tolerance::new(1e-6)).run(2.).unwrap();
        assert!(relative_eq!(f64::sqrt(2.), x))
    }

    #[test]
    fn case02_steffensen() {
        let op = |x: &f64| -> f64 { x * x + x - 2. };
        let solver = Steffensen::new();
        let x = Executor::new(solver, op).terminate(Tolerance::new(1e-6)).run(2.).unwrap();
        assert!(relative_eq!(f64::sqrt(2.), x))
    }
}
