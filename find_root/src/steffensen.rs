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
        let mut x = Vec::from(init);
        for _k in 0..self.max_iter {
            let y = (self.f)(&x);
            let z = (self.f)(&y);
            if x.iter_mut()
                .zip(y.iter())
                .zip(z.iter())
                .zip(atol.iter())
                .zip(rtol.iter())
                .fold(true, |converge, ((((x0, &x1), &x2), &atol), &rtol)| {
                    let dx0 = x1 - *x0;
                    if dx0.abs() < atol + x1.abs().max(x0.abs()) * rtol {
                        *x0 = x1;
                        converge && true
                    } else {
                        let dx1 = x2 - x1;
                        *x0 = *x0 - dx0.powi(2) / (dx1 - dx0);
                        false
                    }
                })
            {
                return x.to_vec();
            }
        }
        x.to_vec()
    }
}
