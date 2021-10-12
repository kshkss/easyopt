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
        let atol = ArrayView1::from(atol);
        let rtol = ArrayView1::from(rtol);
        let mut x_prev = ArrayView1::from(init).to_owned();
        let mut y_prev = Array1::from((self.f)(x_prev.as_slice().unwrap()));
        let mut x = y_prev.clone();
        for _k in 0..self.max_iter {
            let y = Array1::from((self.f)(x.as_slice().unwrap()));
            if Zip::from(&y)
                .and(&x)
                .and(&atol)
                .and(&rtol)
                .all(|&x1, &x2, &atol, &rtol| {
                    (x1 - x2).abs() < atol + x1.abs().max(x2.abs()) * rtol
                })
            {
                return y.to_vec();
            }
            let dy = &y - &y_prev;
            let dx = &x - &x_prev;
            x_prev = x;
            y_prev = y;
            x = Zip::from(&x_prev)
                .and(&y_prev)
                .and(&dx)
                .and(&dy)
                .and(&atol)
                .apply_collect(|&x, &y, &dx, &dy, &atol| {
                    ((dy * x - dx * y) / ((dy - dx).abs() + atol)) * (dy - dx).signum()
                });
        }
        x.to_vec()
    }
}
