use ndarray::prelude::*;
use num_traits::{One, Zero, Float, Num, FromPrimitive};
use crate::traits::*;

#[derive(Debug, Clone)]
pub struct Dual<T, const N: usize> {
    x: T,
    dx: [T; N],
}

impl<T: Copy> Copy for Dual<T, 1> {}
pub type Var1<T> = Dual<T, 1>;

impl<T: Copy> Copy for Dual<T, 2> {}
pub type Var2<T> = Dual<T, 2>;

impl<T: Copy> Copy for Dual<T, 3> {}
pub type Var3<T> = Dual<T, 3>;

impl<T, const N: usize> Dual<T, N>
where
    T: Copy + Zero + One,
{
    pub fn new(var_index: usize, value: T) -> Self {
        assert!(var_index < N);
        let mut v = Self {
            x: value,
            dx: [T::zero(); N],
        };
        v.dx[var_index] = T::one();
        v
    }

    pub fn constant(value: T) -> Self {
        Self {
            x: value,
            dx: [T::zero(); N],
        }
    }

    pub fn val(&self) -> &T {
        &self.x
    }

    pub fn grad(&self) -> ArrayView1<T> {
        ArrayView1::<T>::from(&self.dx)
    }

    pub fn grad_mut(&mut self) -> ArrayViewMut1<T> {
        ArrayViewMut1::<T>::from(&mut self.dx)
    }
}

impl<T, const N: usize> Default for Dual<T, N>
where
    T: Zero + Default + Copy,
{
    fn default() -> Self {
        Self {
            x: T::default(),
            dx: [T::zero(); N],
        }
    }
}

impl<T, const N: usize> From<T> for Dual<T, N>
where
    T: Num + Copy,
{
    fn from(x: T) -> Self {
        Self {
            x,
            dx: [T::zero(); N],
        }
    }
}

impl<'a, T, const N: usize> From<&'a T> for Dual<T, N>
where
    T: Num + Copy,
{
    fn from(item: &'a T) -> Self {
        Self {
            x: item.clone(),
            dx: [T::zero(); N],
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

impl<T, const N: usize> Into<T> for Dual<T, N>
{
    fn into(self) -> T {
        self.x
    }
}
*/

impl<const N: usize> From<Dual<f64, N>> for f64 {
    fn from(item: Dual<f64, N>) -> f64 {
        item.x
    }
}

impl<const N: usize> From<&Dual<f64, N>> for f64 {
    fn from(item: &Dual<f64, N>) -> f64 {
        item.x
    }
}

/*
impl<T, U, const N: usize> From<Dual<T, N>> for Dual<U, N>
where
    U: From<T>,
    [U; N]: Default,
{
    fn from(item: Dual<T, N>) -> Dual<U, N> {
        let mut v: [U; N] = Default::default();
        for (dst, src) in v.iter_mut().zip(item.dx) {
            *dst = src.into();
        }
        Dual::<U, N> {
            x: U::from(item.x),
            dx: v,
        }
    }
}

impl<T, const N: usize> From<Dual<T, N>> for Dual<f64, N>
where
    T: Into<f64>,
{
    fn from(item: Dual<T, N>) -> Dual<f64, N> {
        let mut v = [0.; N];
        for (dst, src) in v.iter_mut().zip(item.dx){
            *dst = src.into();
        }
        Dual::<f64, N> {
            x: item.x.into(),
            dx: v,
        }
    }
}
*/

mod ops;

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
        Ok(Self{
            x,
            dx: [T::zero(); N]
        })
    }
}

impl<T, const N: usize> FromPrimitive for Dual<T, N>
where
    T: FromPrimitive + Copy
{
    fn from_i64(n: i64) -> Option<Self> {
        Some(Self {
            x: T::from_i64(n)?,
            dx: [T::from_i64(0)?; N],
        })
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(Self {
            x: T::from_u64(n)?,
            dx: [T::from_u64(0)?; N],
        })
    }
}

impl<T, const N: usize> NaN for Dual<T, N>
where
    T: Float,
{
    fn nan() -> Self {
        Self {
            x: T::nan(),
            dx: [T::nan(); N],
        }
    }

    fn is_nan(&self) -> bool {
        self.x.is_nan()
    }
}


impl<T, const N: usize> Inf for Dual<T, N>
where
    T: Float,
{
    fn infinity() -> Self {
        Self {
            x: T::infinity(),
            dx: [T::nan(); N],
        }
    }

    fn neg_infinity() -> Self {
        Self {
            x: T::neg_infinity(),
            dx: [T::nan(); N],
        }
    }

     fn is_finite(&self) -> bool {
        self.x.is_finite()
    }

     fn is_infinite(&self) -> bool {
        self.x.is_infinite()
    }
}

mod elf;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case01() -> anyhow::Result<()> {
        let x = Dual::<f64, 3>::new(0, 0.);
        let y = [Dual::<f64, 3>::new(1, 1.), Dual::<f64, 3>::new(2, 10.)];

        let loss = (&x + &y[0]) * &y[1];
        assert_eq!(array![10., 10., 1.].view(), loss.grad());
        assert_eq!((0. + 1.) * 10., *loss.val());

        Ok(())
    }

    #[test]
    fn case02() -> anyhow::Result<()> {
        let x = Dual::<f64, 3>::new(0, 2.);
        let y = [Dual::<f64, 3>::new(1, 1.), Dual::<f64, 3>::new(2, 10.)];

        let loss = &x + (&x + &y[0]) * &y[1];
        assert_eq!(array![11., 10., 3.].view(), loss.grad());
        assert_eq!(2. + (2. + 1.) * 10., *loss.val());

        Ok(())
    }
}
