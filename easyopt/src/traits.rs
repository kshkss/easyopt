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
    fn next_iter(&mut self, op: &T, x: &T::Variable) -> Result<T::Variable, Error>;
}

pub trait Criteria {
    type Variable;
    fn apply(&self, xnew: &Self::Variable, x: &Self::Variable) -> Result<(), f64>;
}

