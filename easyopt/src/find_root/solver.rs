use super::FindRootOp;
use crate::error::*;
use crate::traits::*;
use num_traits::{Float, FromPrimitive, NumRef, RefNum, NumOps};

pub struct NewtonRaphson;

impl<T> Solver<T> for NewtonRaphson
where
    T: FindRootOp,
    T::Variable: Clone + NumRef,
    for<'a> &'a T::Variable: RefNum<T::Variable>,
    T::Scalar: Float + FromPrimitive,
{
    type Variable = T::Variable;
    type ReportArg = T::Variable;

    fn next_iter(&mut self, op: &T, x: &Self::Variable) -> Result<Self::Variable, Error> {
        let f0 = op.apply(x)?;
        let k1 = op.solve_jacobian(&op.jacobian(x)?, &f0)?;
        let xnew = x - &k1;
        Ok(xnew)
    }

    fn init_report<R: Report<Arg = Self::ReportArg, Op = T>>(
        &self,
        report: &mut R,
        op: &T,
        x: &T::Variable,
    ) -> Result<(), Error> {
        report.init(op, x)
    }

    fn update_report<R: Report<Arg = Self::ReportArg, Op = T>>(
        &self,
        report: &mut R,
        op: &T,
        x: &T::Variable,
    ) -> Result<(), Error> {
        report.update(op, x)
    }
}

pub struct Sand;

impl<T> Solver<T> for Sand
where
    T: FindRootOp,
    T::Variable: Clone + NumRef + NumOps<T::Scalar, T::Variable>,
    for<'a> &'a T::Variable: RefNum<T::Variable> + NumOps<T::Scalar, T::Variable>,
    T::Scalar: Float + FromPrimitive,
{
    type Variable = T::Variable;
    type ReportArg = T::Variable;

    fn next_iter(&mut self, op: &T, x: &Self::Variable) -> Result<Self::Variable, Error> {
        let two = T::Scalar::from_i32(2).unwrap();
        let six = T::Scalar::from_i32(6).unwrap();

        let f0 = op.apply(x)?;
        let k1 = op.solve_jacobian(&op.jacobian(x)?, &f0)?;
        let k2 = op.solve_jacobian(&op.jacobian(&(x - &(&k1 * two.recip())))?, &f0)?;
        let k3 = op.solve_jacobian(&op.jacobian(&(x - &(&k2 * two.recip())))?, &f0)?;
        let k4 = op.solve_jacobian(&op.jacobian(&(x - &k3))?, &f0)?;
        let xnew = x - &((k1 + &((k2 + &k3) * two) + &k4) * six.recip());
        Ok(xnew)
    }

    fn init_report<R: Report<Arg = Self::ReportArg, Op = T>>(
        &self,
        report: &mut R,
        op: &T,
        x: &T::Variable,
    ) -> Result<(), Error> {
        report.init(op, x)
    }

    fn update_report<R: Report<Arg = Self::ReportArg, Op = T>>(
        &self,
        report: &mut R,
        op: &T,
        x: &T::Variable,
    ) -> Result<(), Error> {
        report.update(op, x)
    }
}
