use ndarray::prelude::*;
use num_traits::{One, Zero};
use std::marker::PhantomData;

pub struct Variables<T, const MAX_VAR: usize> {
    num_var: usize,
    _type: PhantomData<T>,
}

impl<T, const MAX_VAR: usize> Variables<T, MAX_VAR>
where
    T: One + Zero + Copy,
{
    pub fn new() -> Self {
        Self {
            num_var: 0,
            _type: PhantomData::<T>,
        }
    }

    fn var(i: usize, v: T) -> Dual<T, MAX_VAR> {
        let mut dx = [T::zero(); MAX_VAR];
        dx[i] = T::one();
        Dual::<T, MAX_VAR> { x: v, dx: dx }
    }

    pub fn gen(&mut self, init_value: T) -> Option<Dual<T, MAX_VAR>> {
        if self.num_var < MAX_VAR {
            let var = Self::var(self.num_var, init_value);
            self.num_var += 1;
            Some(var)
        } else {
            None
        }
    }

    pub fn gen_all(&mut self, init_values: &[T]) -> Vec<Dual<T, MAX_VAR>> {
        if init_values.len() == MAX_VAR - self.num_var {
            init_values
                .iter()
                .zip(self.num_var..MAX_VAR)
                .map(|(v, i)| Self::var(i, v.clone()))
                .collect()
        } else {
            panic!();
        }
    }

    pub fn constant(&self, value: T) -> Dual<T, MAX_VAR> {
        Dual::<T, MAX_VAR> {
            x: value,
            dx: [T::zero(); MAX_VAR],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dual<T, const N: usize> {
    x: T,
    dx: [T; N],
}

impl<T, const N: usize> Dual<T, N> {
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

pub mod ops;

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

pub mod elementary;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case01() {
        let mut vars = Variables::<f64, 3>::new();
        let x = vars.gen(0.).unwrap();
        let y = vars.gen_all(&[1., 10.]);

        let loss = (&x + &y[0]) * &y[1];
        assert_eq!(array![10., 10., 1.].view(), loss.grad());
        assert_eq!((0. + 1.) * 10., *loss.val());
    }

    #[test]
    fn case02() {
        let mut vars = Variables::<f64, 3>::new();
        let x = vars.gen(2.).unwrap();
        let y = vars.gen_all(&[1., 10.]);

        let loss = &x + (&x + &y[0]) * &y[1];
        assert_eq!(array![11., 10., 3.].view(), loss.grad());
        assert_eq!(2. + (2. + 1.) * 10., *loss.val());
    }
}
