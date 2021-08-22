pub use crate::criteria::*;
pub use crate::error::*;
pub use crate::executor::*;
pub use crate::monitor;
pub use crate::traits::*;

use num_traits::{Zero, Float, NumRef};
use serde::Serialize;

pub trait FindRootOp {
    type Scalar;
    type Variable;
    type Jacobian;
    fn apply(&self, x: &Self::Variable) -> Result<Self::Variable, Error>;
    fn jacobian(&self, x: &Self::Variable) -> Result<Self::Jacobian, Error>;
    fn solve_jacobian(
        &self,
        a: &Self::Jacobian,
        x: &Self::Variable,
    ) -> Result<Self::Variable, Error>;
}

/*
impl<F> FindRootOp for F
where
    F: Fn(&f64) -> f64,
{
    type Scalar = f64;
    type Variable = f64;
    type Jacobian
    fn apply(&self, x: &f64) -> Result<f64, Error> {
        Ok(self(x))
    }
    fn solve_jacobian(&self, x: &Self::Variable) -> Result<Self::Variable, Error>{
        todo!()
    }
}
*/

#[derive(Serialize)]
pub struct DefaultReport<T>
where
    T: FindRootOp,
{
    pub count: usize,
    pub current: T::Variable,
    pub error: T::Variable,
}

impl<T> Report for DefaultReport<T>
where
    T: FindRootOp,
    T::Variable: Clone + Float,
{
    type Arg = T::Variable;
    type Op = T;

    fn init(&mut self, op: &Self::Op, x: &Self::Arg) -> Result<(), Error> {
        self.count = 0;
        self.current = x.clone();
        self.error = op.apply(x)?.abs();
        Ok(())
    }

    fn update(&mut self, op: &Self::Op, x: &Self::Arg) -> Result<(), Error> {
        self.count += 1;
        self.current = x.clone();
        self.error = op.apply(x)?.abs();
        Ok(())
    }
}

impl<T> Default for DefaultReport<T>
where
    T: FindRootOp,
    T::Variable: Clone + Float,
{
    fn default() -> Self {
        Self {
            count: 0,
            current: Zero::zero(),
            error: Float::nan(),
        }
    }
}

pub struct Executor<S, O> {
    solver: S,
    op: O,
}

impl<S, O> Executor<S, O>
where
    S: Solver<O, Variable = O::Variable, ReportArg = O::Variable>,
    O: FindRootOp,
    O::Variable: Clone + Float,
{
    pub fn new(solver: S, op: O) -> Self {
        Self { solver, op }
    }

    pub fn report<'a, T>(self, report: T) -> ExecutorStage1<'a, S, O, T>
    where
        T: Report<Arg = S::ReportArg, Op = O>,
    {
        ExecutorStage1::<'a, S, O, T> {
            solver: self.solver,
            op: self.op,
            report,
            monitor: Vec::with_capacity(4),
        }
    }

    pub fn add_monitor<'a, F>(self, f: F) -> ExecutorStage1<'a, S, O, DefaultReport<O>>
    where
        F: 'a + Monitor<DefaultReport<O>>,
    {
        self.report(Default::default()).add_monitor(f)
    }

    pub fn terminate<'a, F>(self, c: F) -> ExecutorReady<'a, S, O, DefaultReport<O>, F>
    where
        F: Criteria<DefaultReport<O>>,
    {
        self.report(Default::default()).terminate(c)
    }
}

pub mod solver;

#[cfg(test)]
mod tests {
    use super::*;
    use approx::relative_eq;

    #[test]
    fn case01_newton() -> anyhow::Result<()> {
        struct Foo;
        impl FindRootOp for Foo {
            type Scalar = f64;
            type Variable = f64;
            type Jacobian = f64;

            fn apply(&self, x: &f64) -> Result<f64, Error> {
                Ok(x * x - 2.)
            }

            fn jacobian(&self, x: &f64) -> Result<f64, Error> {
                Ok(2. * x)
            }

            fn solve_jacobian(&self, a: &f64, x: &f64) -> Result<f64, Error> {
                Ok(x / a)
            }
        }

        let op = Foo;
        let solver = solver::NewtonRaphson;
        let x = Executor::new(solver, op)
            .add_monitor(monitor::to_file("case01_newton.log")?)
            .terminate(when(|report: &DefaultReport<_>| report.error < 1e-12))
            .run(2.)?;
        assert!(relative_eq!(f64::sqrt(2.), x));

        Ok(())
    }

    #[test]
    fn case01_sand() -> anyhow::Result<()> {
        struct Foo;
        impl FindRootOp for Foo {
            type Scalar = f64;
            type Variable = f64;
            type Jacobian = f64;

            fn apply(&self, x: &f64) -> Result<f64, Error> {
                Ok(x * x - 2.)
            }

            fn jacobian(&self, x: &f64) -> Result<f64, Error> {
                Ok(2. * x)
            }

            fn solve_jacobian(&self, a: &f64, x: &f64) -> Result<f64, Error> {
                Ok(x / a)
            }
        }

        let op = Foo;
        let solver = solver::Sand;
        let x = Executor::new(solver, op)
            .add_monitor(monitor::to_file("case01_sand.log")?)
            .terminate(when(|report: &DefaultReport<_>| report.error < 1e-12))
            .run(2.)?;
        println!("{} {}", f64::sqrt(2.), x);
        assert!(relative_eq!(f64::sqrt(2.), x));

        Ok(())
    }
}
