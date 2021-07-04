use ndarray::prelude::*;
use ndarray::ScalarOperand;
use num_traits::{One, Zero};
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub};

use super::Dual;

impl<T> PartialEq<Self> for Dual<T>
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

impl<T> PartialEq<T> for Dual<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.x.eq(&other)
    }

    fn ne(&self, other: &T) -> bool {
        self.x.ne(&other)
    }
}

impl<T> PartialOrd<Self> for Dual<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl<T> PartialOrd<T> for Dual<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.x.partial_cmp(&other)
    }
}

impl<T> Add<Self> for Dual<T>
where
    T: Zero + Add + Clone,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            dx: ArcArray::from(&self.dx + &rhs.dx),
        }
    }
}

impl<T> Add<T> for Dual<T>
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

impl<T> Sub<Self> for Dual<T>
where
    T: Zero + Sub<Output = T> + Clone,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            dx: ArcArray::from(&self.dx - &rhs.dx),
        }
    }
}

impl<T> Sub<T> for Dual<T>
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

impl<T> Neg for Dual<T>
where
    T: Zero + Neg<Output = T> + Sub<Output = T> + Clone,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            dx: ArcArray::from(-self.dx.to_owned()),
        }
    }
}

impl<T> Mul<Self> for Dual<T>
where
    T: Zero + One + Add + Mul + ScalarOperand + Clone,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.clone() * rhs.x.clone(),
            dx: ArcArray::from(&self.dx * rhs.x + &rhs.dx * self.x),
        }
    }
}

impl<T> Mul<T> for Dual<T>
where
    T: Zero + One + Add + Mul + ScalarOperand + Clone,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs.clone(),
            dx: ArcArray::from(&self.dx * rhs),
        }
    }
}

impl<T> Div<Self> for Dual<T>
where
    T: Zero + One + Add + Sub<Output = T> + Mul + Div<Output = T> + ScalarOperand + Clone,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            dx: ArcArray::from(
                &self.dx * rhs.x.clone()
                    - &rhs.dx * self.x.clone() / (rhs.x.clone() * rhs.x.clone()),
            ),
            x: self.x / rhs.x,
        }
    }
}

impl<T> Div<T> for Dual<T>
where
    T: Zero + One + Add + Sub<Output = T> + Mul + Div<Output = T> + ScalarOperand + Clone,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            dx: ArcArray::from(&self.dx / rhs.clone()),
            x: self.x / rhs,
        }
    }
}

