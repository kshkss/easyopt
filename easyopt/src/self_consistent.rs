use crate::traits::*;
use crate::Error;
use num_traits::Zero;
use serde::Serialize;

pub trait SelfConsistentOp {
    type Variable;
    fn apply(&self, x: &Self::Variable) -> Result<Self::Variable, Error>;
}

impl<T> Op for T
where
    T: SelfConsistentOp,
{
    type Variable = <T as SelfConsistentOp>::Variable;
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

    fn init(&mut self, x: &Self::Arg) -> Result<(), Error> {
        self.count = 0;
        self.current = x.clone();
        self.error = Float::nan();
        self.prev = None;
        Ok(())
    }

    fn update(&mut self, x: &Self::Arg) -> Result<(), Error> {
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

pub mod solver;
use solver::SelfConsistentOpSolver;

impl<S, O> super::Executor<S, O>
where
    S: SelfConsistentOpSolver<O>,
    O: SelfConsistentOp,
    O::Variable: Clone + Float + for<'a> BinaryOperand<&'a O::Variable, O::Variable>,
    for<'a, 'b> &'a O::Variable: BinaryOperand<&'b O::Variable, O::Variable>,
{
    pub fn add_monitor<'a, F>(self, f: F) -> super::ExecutorStage1<'a, S, O, DefaultReport<O>>
    where
        F: 'a + Monitor<DefaultReport<O>>,
    {
        self.report(Default::default()).add_monitor(f)
    }

    pub fn terminate<'a, F>(self, c: F) -> super::ExecutorReady<'a, S, O, DefaultReport<O>, F>
    where
        F: Criteria<DefaultReport<O>>,
    {
        self.report(Default::default()).terminate(c)
    }
}
