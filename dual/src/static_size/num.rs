use super::Dual;
use num_traits::{Num, One, Zero};
use std::cmp::Ordering;

impl<T, const N: usize> PartialEq<Self> for Dual<T, N>
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

impl<T, const N: usize> PartialEq<T> for Dual<T, N>
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

impl<T, const N: usize> PartialOrd<Self> for Dual<T, N>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl<T, const N: usize> PartialOrd<T> for Dual<T, N>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.x.partial_cmp(&other)
    }
}

impl<T, const N: usize> Zero for Dual<T, N>
where
    T: Zero + Copy + PartialEq<T>,
{
    fn zero() -> Self {
        Self {
            x: T::zero(),
            dx: [T::zero(); N],
        }
    }

    fn is_zero(&self) -> bool {
        self.x == T::zero()
    }

    fn set_zero(&mut self) {
        self.x = T::zero();
        for dst in self.dx.iter_mut() {
            *dst = T::zero();
        }
    }
}

impl<T, const N: usize> One for Dual<T, N>
where
    T: Zero + One + PartialEq<T> + Copy,
{
    fn one() -> Self {
        Self {
            x: T::one(),
            dx: [T::zero(); N],
        }
    }

    fn set_one(&mut self) {
        self.x = T::one();
        for dst in self.dx.iter_mut() {
            *dst = T::one();
        }
    }

    fn is_one(&self) -> bool {
        self.x == T::one()
    }
}

impl<T, const N: usize> Num for Dual<T, N>
where
    T: Num + Copy,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        let x = T::from_str_radix(str, radix)?;
        Ok(Self {
            x,
            dx: [T::zero(); N],
        })
    }
}
