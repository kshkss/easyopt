use ndarray::prelude::*;
use ndarray::ScalarOperand;
use num_traits::{Num, NumOps, One, Zero};
use std::cmp::Ordering;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub struct Variables<T = f64> {
    max_var: usize,
    num_var: usize,
    _type: PhantomData<T>,
}

impl<T> Variables<T>
where
    T: One + Zero + Clone,
{
    pub fn new(num_variables: usize) -> Self {
        Self {
            max_var: num_variables,
            num_var: 0,
            _type: PhantomData::<T>,
        }
    }

    fn var(i: usize, n: usize, v: T) -> Dual<T> {
        let mut dx = Array1::<T>::zeros(n);
        dx[i] = T::one();
        Dual::<T> {
            x: v,
            dx: ArcArray::from(dx),
        }
    }

    pub fn gen(&mut self, init_value: T) -> Option<Dual<T>> {
        if self.num_var < self.max_var {
            let var = Self::var(self.num_var, self.max_var, init_value);
            self.num_var += 1;
            Some(var)
        } else {
            None
        }
    }

    pub fn gen_all(&mut self, init_values: &[T]) -> Vec<Dual<T>> {
        if init_values.len() == self.max_var - self.num_var {
            init_values
                .iter()
                .zip(self.num_var..self.max_var)
                .map(|(v, i)| Self::var(i, self.max_var, v.clone()))
                .collect()
        } else {
            panic!();
        }
    }

    pub fn constant(&self, value: T) -> Dual<T> {
        Dual::<T> {
            x: value,
            dx: ArcArray::from(Array1::<T>::zeros(self.max_var)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dual<T = f64> {
    x: T,
    dx: ArcArray<T, Ix1>,
}

impl<T> Dual<T>
where
    T: Clone,
{
    pub fn grad(&self) -> Option<ArrayView1<T>> {
        Some(self.dx.view())
    }

    pub fn grad_mut(&mut self) -> Option<ArrayViewMut1<T>> {
        Some(self.dx.view_mut())
    }
}

/*
impl<T> From<Dual<T>> for T
{
    fn from(item: Dual<T>) -> T {
        item.x
    }
}

impl<T> Into<T> for Dual<T>
{
    fn into(self) -> T {
        self.x
    }
}
*/

impl From<Dual<f64>> for f64 {
    fn from(item: Dual<f64>) -> f64 {
        item.x
    }
}

/*
impl<T, U> From<Dual<T>> for Dual<U>
where
    U: From<T>,
{
    fn from(item: Dual<T>) -> Dual<U> {
        Dual::<U> {
            x: U::from(item.x),
            dx: item.dx.map(|&v| U::from(v)),
        }
    }
}

impl<T> From<Dual<T>> for Dual<f64>
where
    T: Into<f64>,
{
    fn from(item: Dual<T>) -> Dual<f64> {
        Dual::<f64> {
            x: item.x.into(),
            dx: item.dx.map(|&v| v.into()),
        }
    }
}
*/

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut vars = Variables::new(3);
        let x = vars.gen(0.).unwrap();
        let rest = vars.gen_all(&[1., 10.]);
        let y = rest[0].clone();
        let z = rest[1].clone();

        let loss = (x + y) * z;
        assert_eq!(loss.grad(), Some(array![10., 10., 1.].view()));
    }
}
