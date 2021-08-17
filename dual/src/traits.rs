use num_traits::Float;
use std::ops::{Add, Div, Mul, Sub};

pub trait NaN {
    fn nan() -> Self;
    fn is_nan(&self) -> bool;
}

impl<T> NaN for T
where
    T: Float,
{
    fn nan() -> Self {
        Float::nan()
    }

    fn is_nan(&self) -> bool {
        Float::is_nan(*self)
    }
}

pub trait Inf {
    fn infinity() -> Self;
    fn neg_infinity() -> Self;
    fn is_infinite(&self) -> bool;
    fn is_finite(&self) -> bool;
}

impl<T> Inf for T
where
    T: Float,
{
    fn infinity() -> Self {
        Float::infinity()
    }

    fn neg_infinity() -> Self {
        Float::neg_infinity()
    }

    fn is_infinite(&self) -> bool {
        Float::is_infinite(*self)
    }

    fn is_finite(&self) -> bool {
        Float::is_finite(*self)
    }
}

pub trait ArithmeticOperation<Rhs, Output>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
{
}

impl<T, Rhs, Output> ArithmeticOperation<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
{
}

pub trait ElementaryFunction {
    fn negate(&self) -> Self;
    fn recip(&self) -> Self;

    fn sqrt(&self) -> Self;
    fn cbrt(&self) -> Self;
    fn powf(&self, n: &Self) -> Self;
    fn powi(&self, n: i32) -> Self;

    fn exp(&self) -> Self;
    fn exp2(&self) -> Self;
    fn exp_m1(&self) -> Self;

    fn ln(&self) -> Self;
    fn log10(&self) -> Self;
    fn log2(&self) -> Self;
    fn log(&self, base: &Self) -> Self;
    fn ln_1p(&self) -> Self;

    fn cos(&self) -> Self;
    fn sin(&self) -> Self;
    fn tan(&self) -> Self;

    fn acos(&self) -> Self;
    fn asin(&self) -> Self;
    fn atan(&self) -> Self;

    fn cosh(&self) -> Self;
    fn sinh(&self) -> Self;
    fn tanh(&self) -> Self;

    fn acosh(&self) -> Self;
    fn asinh(&self) -> Self;
    fn atanh(&self) -> Self;
}
