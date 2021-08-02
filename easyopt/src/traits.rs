use num_traits::{One, Zero};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait NumOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Neg<Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
{
}

impl<T, Rhs, Output> NumOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Neg<Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
{
}

pub trait BinaryOperand<Rhs, Output>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
{
}

impl<T, Rhs, Output> BinaryOperand<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
{
}

pub trait Float: Zero + One {
    type Scalar;
    fn nan() -> Self;
    fn abs(&self) -> Self;
}

impl Float for f64 {
    type Scalar = f64;

    #[inline]
    fn nan() -> Self {
        std::f64::NAN
    }

    #[inline]
    fn abs(&self) -> Self {
        f64::abs(*self)
    }
}

/*
pub trait ScalarOperand<T>: BinaryOperand<T, T> + for<'a> BinaryOperand<&'a T, T>
{
}

impl<T, K> ScalarOperand<T> for K
where
    K: BinaryOperand<T, T> + for<'a> BinaryOperand<&'a T, T>,
{
}
*/

pub trait Extension<K>:
    Clone + Zero + One + NumOps<Self, Self> + for<'a> NumOps<&'a Self, Self> + NumOps<K, Self>
where
    K: Copy + Zero + One + NumOps<K, K> + for<'a> NumOps<&'a K, K>,
    for<'a, 'b> &'a K: NumOps<K, K> + NumOps<&'b K, K>,
    for<'a, 'b> &'a Self: NumOps<Self, Self> + NumOps<&'b Self, Self> + NumOps<K, Self>,
{
}

impl Extension<f32> for f32 {}
impl Extension<f64> for f64 {}

use crate::Error;

pub trait Op {
    type Variable;
}

pub trait Solver<T>
where
    T: Op,
{
    type ReportArg;
    fn next_iter(&mut self, op: &T, x: &T::Variable) -> Result<T::Variable, Error>;
    fn init_report<R: Report<Arg = Self::ReportArg>>(
        &self,
        report: &mut R,
        x: &T::Variable,
    ) -> Result<(), Error>;
    fn update_report<R: Report<Arg = Self::ReportArg>>(
        &self,
        report: &mut R,
        x: &T::Variable,
    ) -> Result<(), Error>;
}

pub trait Report {
    type Arg;
    fn init(&mut self, s: &Self::Arg) -> Result<(), Error>;
    fn update(&mut self, s: &Self::Arg) -> Result<(), Error>;
}

pub trait Monitor<T>: FnMut(&T) -> anyhow::Result<()> {}
impl<T: Report, F> Monitor<T> for F where F: FnMut(&T) -> anyhow::Result<()> {}

pub trait Criteria<T>: Fn(&T) -> Result<(), f64> {}
impl<T: Report, F> Criteria<T> for F where F: Fn(&T) -> Result<(), f64> {}
