use crate::traits::Op;
use crate::Error;

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

pub mod solver;

