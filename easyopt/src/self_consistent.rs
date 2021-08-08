pub use crate::criteria::*;
pub use crate::error::*;
pub use crate::executor::*;
pub use crate::monitor;
pub use crate::traits::*;

use num_traits::Zero;
use serde::Serialize;

pub trait SelfConsistentOp {
    type Variable;
    fn apply(&self, x: &Self::Variable) -> Result<Self::Variable, Error>;
}

/* Bad
impl<T> SelfConsistentOp for dyn Fn(&T) -> T {
    type Variable = T;
    fn apply(&self, x: &T) -> Result<T, Error> {
        Ok(self(x))
    }
}
*/
/* Bad
impl<F, T> SelfConsistentOp for F
where F: Fn(&T) -> T {
    type Variable = T;
    fn apply(&self, x: &T) -> Result<T, Error> {
        Ok(self(x))
    }
}
*/
impl<F> SelfConsistentOp for F
where
    F: Fn(&f64) -> f64,
{
    type Variable = f64;
    fn apply(&self, x: &f64) -> Result<f64, Error> {
        Ok(self(x))
    }
}

#[derive(Serialize)]
pub struct DefaultReport<T>
where
    T: SelfConsistentOp,
{
    pub count: usize,
    pub current: T::Variable,
    pub error: T::Variable,
    #[serde(skip)]
    prev: Option<T::Variable>,
}

impl<T> Report for DefaultReport<T>
where
    T: SelfConsistentOp,
    T::Variable: Clone + Float + for<'a> BinaryOperand<&'a T::Variable, T::Variable>,
    for<'a, 'b> &'b T::Variable: Clone + BinaryOperand<&'a T::Variable, T::Variable>,
{
    type Arg = T::Variable;
    type Op = T;

    fn init(&mut self, _op: &Self::Op, x: &Self::Arg) -> Result<(), Error> {
        self.count = 0;
        self.current = x.clone();
        self.error = Float::nan();
        self.prev = None;
        Ok(())
    }

    fn update(&mut self, _op: &Self::Op, x: &Self::Arg) -> Result<(), Error> {
        let prev = std::mem::replace(&mut self.current, x.clone());
        self.error = (&self.current - &prev).abs() / &prev;
        self.prev = Some(prev);
        self.count += 1;
        Ok(())
    }
}

impl<T> Default for DefaultReport<T>
where
    T: SelfConsistentOp,
    T::Variable: Clone + Float,
{
    fn default() -> Self {
        Self {
            count: 0,
            current: Zero::zero(),
            error: Float::nan(),
            prev: None,
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
    O: SelfConsistentOp,
    O::Variable: Clone + Float + for<'a> BinaryOperand<&'a O::Variable, O::Variable>,
    for<'a, 'b> &'a O::Variable: BinaryOperand<&'b O::Variable, O::Variable>,
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
mod test {
    use super::*;
    use approx::relative_eq;

    #[test]
    fn case01_wegstein() -> anyhow::Result<()> {
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

        let op = TestCase01 {
            a: 1.,
            b: 1.,
            c: -2.,
        };
        let solver = solver::Wegstein::<f64>::new();
        let x = Executor::new(solver, op)
            .report(DefaultReport::<TestCase01>::default())
            .add_monitor(monitor::to_file("test.log")?)
            .terminate(when(|report: &DefaultReport<_>| report.error < 1e-8))
            .run(2.)?;
        assert!(relative_eq!(f64::sqrt(2.), x));

        Ok(())
    }

    #[test]
    fn case02_wegstein() -> anyhow::Result<()> {
        let op = |x: &f64| -> f64 { x * x + x - 2. };
        let solver = solver::Wegstein::<f64>::new();
        let x = Executor::new(solver, op)
            .add_monitor(monitor::to_file("case02_wegstein.log")?)
            .terminate(when(|report: &DefaultReport<_>| report.error < 1e-8))
            .run(2.)?;
        assert!(relative_eq!(f64::sqrt(2.), x));

        Ok(())
    }

    #[test]
    fn case02_steffensen() -> anyhow::Result<()> {
        let op = |x: &f64| -> f64 { x * x + x - 2. };
        let solver = solver::Steffensen::new();
        let x = Executor::new(solver, op)
            .terminate(when(|report: &DefaultReport<_>| report.error < 1e-8))
            .add_monitor(monitor::to_file("case02_steffensen.log")?)
            .run(2.)?;
        assert!(relative_eq!(f64::sqrt(2.), x));
        Ok(())
    }
}
