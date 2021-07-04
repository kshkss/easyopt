use ndarray::prelude::*;
use ndarray::ScalarOperand;
use num_traits::{Num, NumOps, One, Zero};
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Debug)]
pub struct Dual<T, const N: usize> {
    x: T,
    dx: Array1<T>,
}

impl<T, const N: usize> Dual<T, N>
where
    T: One + Zero + Clone,
{
    pub fn new(variable_id: usize, init_value: T) -> Self {
        if variable_id < N {
            let mut dx = Array1::<T>::zeros(N);
            dx[variable_id] = T::one();
            Self {
                x: init_value,
                dx: dx,
            }
        } else {
            panic!(
                "variable_id is expected under {}, but found {}",
                N, variable_id
            );
        }
    }

    pub fn constant(value: T) -> Self {
        Self {
            x: value,
            dx: Array1::<T>::zeros(N),
        }
    }

    pub fn grad(&self) -> Option<ArrayView1<T>> {
        Some(self.dx.view())
    }

    pub fn grad_mut(&mut self) -> Option<ArrayViewMut1<T>> {
        Some(self.dx.view_mut())
    }
}

impl<T, const N: usize> Default for Dual<T, N>
where
    T: Zero + Clone,
{
    fn default() -> Self {
        Self {
            x: T::zero(),
            dx: Array1::<T>::zeros(N),
        }
    }
}

/*
impl<T, const N: usize> From<Dual<T, N>> for T
{
    fn from(item: Dual<T, N>) -> T {
        item.x
    }
}
*/

/*
impl<T, const N: usize> Into<T> for Dual<T, N>
{
    fn into(self) -> T {
        self.x
    }
}
*/

impl<const N: usize> From<Dual<f64, N>> for f64
{
    fn from(item: Dual<f64, N>) -> f64 {
        item.x
    }
}

/*
impl<T, U, const N: usize> From<Dual<T, N>> for Dual<U, N>
where
    U: From<T>,
{
    fn from(item: Dual<T, N>) -> Dual<U, N> {
        Dual::<U, N> {
            x: U::from(item.x),
            dx: item.dx.map(|&v| U::from(v)),
        }
    }
}
*/

/*
impl<T, const N: usize> From<Dual<T, N>> for Dual<f64, N>
where
    T: Into<f64>,
{
    fn from(item: Dual<T, N>) -> Dual<f64, N> {
        Dual::<f64, N> {
            x: item.x.into(),
            dx: item.dx.map(|&v| v.into()),
        }
    }
}
*/

impl<T, const N: usize> PartialEq for Dual<T, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x)
    }

    fn ne(&self, other: &Self) -> bool {
        self.x.ne(&other.x)
    }
}

impl<T, const N: usize> PartialOrd for Dual<T, N>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl<T, const N: usize> Zero for Dual<T, N>
where
    T: Zero + Clone,
{
    fn zero() -> Self {
        Self {
            x: T::zero(),
            dx: Array1::<T>::zeros(N),
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero()
    }
}

impl<T, const N: usize> One for Dual<T, N>
where
    T: Zero + One + ScalarOperand + Clone,
{
    fn one() -> Self {
        Self {
            x: T::one(),
            dx: Array1::<T>::zeros(N),
        }
    }
}

impl<T, const N: usize> Add<Self> for Dual<T, N>
where
    T: Zero + Add + Clone,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            dx: &self.dx + &rhs.dx,
        }
    }
}

impl<T, const N: usize> Add<T> for Dual<T, N>
where
    T: Zero + Add + Clone,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        Self {
            x: self.x + rhs,
            dx: self.dx,
        }
    }
}

impl<T, const N: usize> Sub<Self> for Dual<T, N>
where
    T: Zero + Sub<Output = T> + Clone,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            dx: &self.dx - &rhs.dx,
        }
    }
}

impl<T, const N: usize> Sub<T> for Dual<T, N>
where
    T: Zero + Sub<Output = T> + Clone,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        Self {
            x: self.x - rhs,
            dx: self.dx,
        }
    }
}

impl<T, const N: usize> Neg for Dual<T, N>
where
    T: Zero + Neg<Output = T> + Sub<Output = T> + Clone,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            dx: -self.dx,
        }
    }
}

impl<T, const N: usize> Mul<Self> for Dual<T, N>
where
    T: Zero + One + Add + Mul + ScalarOperand + Clone,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.clone() * rhs.x.clone(),
            dx: &self.dx * rhs.x + &rhs.dx * self.x,
        }
    }
}

impl<T, const N: usize> Mul<T> for Dual<T, N>
where
    T: Zero + One + Add + Mul + ScalarOperand + Clone,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs.clone(),
            dx: &self.dx * rhs,
        }
    }
}

impl<T, const N: usize> Div<Self> for Dual<T, N>
where
    T: Zero + One + Add + Sub<Output = T> + Mul + Div<Output = T> + ScalarOperand + Clone,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            dx: &self.dx * rhs.x.clone() - &rhs.dx * self.x.clone() / (rhs.x.clone() * rhs.x.clone()),
            x: self.x / rhs.x,
        }
    }
}

impl<T, const N: usize> Div<T> for Dual<T, N>
where
    T: Zero + One + Add + Sub<Output = T> + Mul + Div<Output = T> + ScalarOperand + Clone,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            dx: &self.dx / rhs.clone(),
            x: self.x / rhs,
        }
    }
}
