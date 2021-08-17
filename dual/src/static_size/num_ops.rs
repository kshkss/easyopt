use num_traits::{Num, NumAssignOps, NumOps};
use num_traits::{One, Zero};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use super::Dual;

impl<T, const N: usize> Add<&Self> for Dual<T, N>
where
    T: Zero + Add + Copy,
{
    type Output = Dual<T, N>;
    fn add(mut self, rhs: &Self) -> Self::Output {
        for (dst, &src) in self.dx.iter_mut().zip(rhs.dx.iter()) {
            *dst = *dst + src;
        }
        self.x = self.x + rhs.x;
        self
    }
}

impl<T, const N: usize> Add<Self> for Dual<T, N>
where
    T: Zero + Add + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        self + &rhs
    }
}

impl<'a, 'b, T, const N: usize> Add<&'a Dual<T, N>> for &'b Dual<T, N>
where
    T: Zero + Add + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn add(self, rhs: &'a Dual<T, N>) -> Self::Output {
        self.clone() + rhs
    }
}

impl<T, const N: usize> Add<Dual<T, N>> for &Dual<T, N>
where
    T: Zero + Add + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn add(self, rhs: Dual<T, N>) -> Self::Output {
        rhs + self
    }
}

impl<T, const N: usize> Add<T> for Dual<T, N>
where
    T: Add<Output = T> + Copy,
{
    type Output = Dual<T, N>;
    fn add(mut self, rhs: T) -> Self::Output {
        self.x = self.x + rhs;
        self
    }
}

impl<T, const N: usize> Add<T> for &Dual<T, N>
where
    T: Add<Output = T> + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        self.clone() + rhs
    }
}

impl<T, const N: usize> Neg for Dual<T, N>
where
    T: Zero + Neg<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        for dst in self.dx.iter_mut() {
            *dst = -*dst;
        }
        self.x = -self.x;
        self
    }
}

impl<T, const N: usize> Neg for &Dual<T, N>
where
    T: Zero + Neg<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Dual<T, N>;
    fn neg(self) -> Self::Output {
        -self.clone()
    }
}

impl<T, const N: usize> Sub<&Self> for Dual<T, N>
where
    T: Zero + Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(mut self, rhs: &Self) -> Self::Output {
        for (dst, &src) in self.dx.iter_mut().zip(rhs.dx.iter()) {
            *dst = *dst - src;
        }
        self.x = self.x - rhs.x;
        self
    }
}

impl<T, const N: usize> Sub<Self> for Dual<T, N>
where
    T: Zero + Sub<Output = T> + Copy,
{
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self - &rhs
    }
}

impl<'a, 'b, T, const N: usize> Sub<&'a Dual<T, N>> for &'b Dual<T, N>
where
    T: Zero + Sub<Output = T> + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn sub(self, rhs: &'a Dual<T, N>) -> Self::Output {
        self.clone() - rhs
    }
}

impl<T, const N: usize> Sub<Dual<T, N>> for &Dual<T, N>
where
    T: Zero + Sub<Output = T> + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn sub(self, rhs: Dual<T, N>) -> Self::Output {
        self.clone() - &rhs
    }
}

impl<T, const N: usize> Sub<T> for Dual<T, N>
where
    T: Zero + Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(mut self, rhs: T) -> Self::Output {
        self.x = self.x - rhs;
        self
    }
}

impl<T, const N: usize> Sub<T> for &Dual<T, N>
where
    T: Zero + Sub<Output = T> + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        self.clone() - rhs
    }
}

impl<T, const N: usize> Mul<&Self> for Dual<T, N>
where
    T: Zero + One + Add + Mul + Copy,
{
    type Output = Dual<T, N>;
    fn mul(mut self, rhs: &Self) -> Self::Output {
        for (dst, &src) in self.dx.iter_mut().zip(rhs.dx.iter()) {
            *dst = *dst * rhs.x + src * self.x;
        }
        self.x = self.x * rhs.x;
        self
    }
}

impl<T, const N: usize> Mul<Self> for Dual<T, N>
where
    T: Zero + One + Add + Mul + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self * &rhs
    }
}

impl<'a, 'b, T, const N: usize> Mul<&'a Dual<T, N>> for &'b Dual<T, N>
where
    T: Zero + One + Add + Mul + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn mul(self, rhs: &'a Dual<T, N>) -> Self::Output {
        self.clone() * rhs
    }
}

impl<T, const N: usize> Mul<Dual<T, N>> for &Dual<T, N>
where
    T: Zero + One + Add + Mul + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn mul(self, rhs: Dual<T, N>) -> Self::Output {
        rhs * self
    }
}

impl<T, const N: usize> Mul<T> for Dual<T, N>
where
    T: Zero + One + Add + Mul + Copy,
{
    type Output = Dual<T, N>;
    fn mul(mut self, rhs: T) -> Self::Output {
        for dst in self.dx.iter_mut() {
            *dst = *dst * rhs;
        }
        self.x = self.x * rhs;
        self
    }
}

impl<T, const N: usize> Mul<T> for &Dual<T, N>
where
    T: Zero + One + Add + Mul + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        self.clone() * rhs
    }
}

impl<T, const N: usize> Div<&Self> for Dual<T, N>
where
    T: Zero + One + Add + Sub<Output = T> + Mul + Div<Output = T> + Copy,
{
    type Output = Dual<T, N>;
    fn div(mut self, rhs: &Self) -> Self::Output {
        for (dst, src) in self.dx.iter_mut().zip(rhs.dx) {
            *dst = *dst / rhs.x - src * self.x / (rhs.x * rhs.x);
        }
        self.x = self.x / rhs.x;
        self
    }
}

impl<T, const N: usize> Div<Self> for Dual<T, N>
where
    T: Zero + One + Add + Sub<Output = T> + Mul + Div<Output = T> + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        self / &rhs
    }
}

impl<'a, 'b, T, const N: usize> Div<&'a Dual<T, N>> for &'b Dual<T, N>
where
    T: Zero + One + Add + Sub<Output = T> + Mul + Div<Output = T> + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn div(self, rhs: &'a Dual<T, N>) -> Self::Output {
        self.clone() / rhs
    }
}

impl<T, const N: usize> Div<Dual<T, N>> for &Dual<T, N>
where
    T: Zero + One + Add + Sub<Output = T> + Mul + Div<Output = T> + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn div(self, rhs: Dual<T, N>) -> Self::Output {
        self.clone() / &rhs
    }
}

impl<T, const N: usize> Div<T> for Dual<T, N>
where
    T: Zero + One + Add + Sub<Output = T> + Mul + Div<Output = T> + Copy,
{
    type Output = Dual<T, N>;
    fn div(mut self, rhs: T) -> Self::Output {
        for dst in self.dx.iter_mut() {
            *dst = *dst / rhs;
        }
        self.x = self.x / rhs;
        self
    }
}

impl<T, const N: usize> Div<T> for &Dual<T, N>
where
    T: Zero + One + Add + Sub<Output = T> + Mul + Div<Output = T> + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        self.clone() / rhs
    }
}

impl<T, const N: usize> Rem<&Self> for Dual<T, N>
where
    T: Num + Copy,
{
    type Output = Dual<T, N>;
    fn rem(mut self, rhs: &Self) -> Self::Output {
        for (dst, src) in self.dx.iter_mut().zip(rhs.dx) {
            *dst = todo!();
        }
        self.x = self.x % rhs.x;
        self
    }
}

impl<T, const N: usize> Rem<Self> for Dual<T, N>
where
    T: Num + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        self / &rhs
    }
}

impl<'a, 'b, T, const N: usize> Rem<&'a Dual<T, N>> for &'b Dual<T, N>
where
    T: Num + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn rem(self, rhs: &'a Dual<T, N>) -> Self::Output {
        self.clone() / rhs
    }
}

impl<T, const N: usize> Rem<Dual<T, N>> for &Dual<T, N>
where
    T: Num + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn rem(self, rhs: Dual<T, N>) -> Self::Output {
        self.clone() / &rhs
    }
}

impl<T, const N: usize> Rem<T> for Dual<T, N>
where
    T: Num + Copy,
{
    type Output = Dual<T, N>;
    fn rem(mut self, rhs: T) -> Self::Output {
        for dst in self.dx.iter_mut() {
            *dst = *dst / rhs;
        }
        self.x = self.x / rhs;
        self
    }
}

impl<T, const N: usize> Rem<T> for &Dual<T, N>
where
    T: Num + Copy,
{
    type Output = Dual<T, N>;
    #[inline]
    fn rem(self, rhs: T) -> Self::Output {
        self.clone() / rhs
    }
}
