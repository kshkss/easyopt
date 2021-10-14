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
/// let sol = find_root::Wegstein::new(f).solve(&x0, &[0.], &[1e-15]);
///
/// approx::assert_relative_eq!(2.0_f64.sqrt(), sol[0], max_relative=1e-15);
/// ```
pub struct Wegstein<'a, T> {
    f: Box<dyn 'a + Fn(&[T]) -> Vec<T>>,
    max_iter: usize,
}

impl<'a, T> Wegstein<'a, T>
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
        let mut x_prev = Vec::from(init);
        let mut y_prev = (self.f)(&x_prev);
        let mut x = y_prev.clone();
        for _k in 0..self.max_iter {
            let mut y = (self.f)(&x);
            if x.iter()
                .zip(x_prev.iter_mut())
                .zip(y.iter())
                .zip(y_prev.iter())
                .zip(atol.iter())
                .zip(rtol.iter())
                .fold(
                    true,
                    |converge, (((((&x1, x0), &y1), &y0), &atol), &rtol)| {
                        let dx = x1 - *x0;
                        let dy = y1 - y0;
                        if (y1 - x1).abs() < atol + x1.abs().max(y1.abs()) * rtol {
                            *x0 = y1;
                            converge && true
                        } else {
                            *x0 = (dy * x1 - dx * y1) / (dy - dx);
                            false
                        }
                    },
                )
            {
                return x_prev.to_vec();
            } else {
                std::mem::swap(&mut x, &mut x_prev);
                std::mem::swap(&mut y, &mut y_prev);
            }
        }
        x.to_vec()
    }
}
