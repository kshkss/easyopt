use super::Dual;
use crate::traits::*;
/*
use num_traits::{Float, Num};

impl<T, const N: usize> Num for Dual<T, N>
where
    T: Num + Copy,
{
    type FromStrRadixErr = T::FromStrRadixErr;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(Self {
            x: T::from_str_radix(s, radix)?,
            dx: [T::zero(); N],
        })
    }
}

impl<T, const N: usize> Float for Dual<T, N>
where
    T: Float,
{
    fn nan() -> Self {
        Self {
            x: T::nan(),
            dx: [T::zero(); N],
        }
    }
}
*/
