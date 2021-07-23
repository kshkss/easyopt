use ndarray::prelude::*;
use num_traits::{One, Zero};
use std::marker::PhantomData;

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

mod ops;
pub use self::ops::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works() {
        let mut vars = Variables::new(3);
        let x = vars.gen(0.).unwrap();
        let y = vars.gen_all(&[1., 10.]);

        let loss = &(&x + &y[0]) * &y[1];
        assert_eq!(loss.grad(), Some(array![10., 10., 1.].view()));
    }
}
