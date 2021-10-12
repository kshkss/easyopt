use ndarray::prelude::*;
use ndarray::Zip;
use num_traits::Float;
use std::ops::Mul;

/// Solves self consistent equation.
///
/// Each equation in the system has the form:
///
/// > *x = f(x)*
///
/// The function expects the function *f* outputs the same size of Vec as the input slice
/// Initial state is given in `x0`.
///
/// # Example
///
/// ```
/// let x0 = [2.0];
/// let f = |x: &[f64]| {
///     vec![x[0].powi(2) + x[0] - 2.]
///     };
/// let sol = find_root::Steffensen::new(f).solve(&x0, &[0.], &[1e-15]);
///
/// approx::assert_relative_eq!(2.0_f64.sqrt(), sol[0], max_relative=1e-15);
/// ```
pub struct Steffensen<'a, T> {
    f: Box<dyn 'a + Fn(&[T]) -> Vec<T>>,
    max_iter: usize,
}

impl<'a, T> Steffensen<'a, T>
where
    T: Float + Mul<f64, Output = T>,
{
    pub fn new(f: impl 'a + Fn(&[T]) -> Vec<T>) -> Self {
        Self {
            f: Box::new(f),
            max_iter: 500,
        }
    }

    pub fn with_max_iteration(self, n: usize) -> Self {
        Self {
            max_iter: n,
            ..self
        }
    }

    pub fn solve(&self, init: &[T], atol: &[T], rtol: &[f64]) -> Vec<T> {
        let atol = ArrayView1::from(atol);
        let rtol = ArrayView1::from(rtol);
        let mut x = ArrayView1::from(init).to_owned();
        for _k in 0..self.max_iter {
            let y = Array1::from((self.f)(x.as_slice().unwrap()));
            let z = Array1::from((self.f)(y.as_slice().unwrap()));
            if Zip::from(&z)
                .and(&x)
                .and(&atol)
                .and(&rtol)
                .all(|&x1, &x2, &atol, &rtol| {
                    (x1 - x2).abs() < atol + x1.abs().max(x2.abs()) * rtol
                })
            {
                return z.to_vec();
            }
            let dx0 = &y - &x;
            let dx1 = &z - &y;
            x = Zip::from(&x).and(&dx0).and(&dx1).and(&atol).apply_collect(
                |&x0, &dx0, &dx1, &atol| {
                    x0 - dx0.powi(2) / ((dx1 - dx0).abs() + atol) * (dx1 - dx0).signum()
                },
            );
        }
        x.to_vec()
    }
}
