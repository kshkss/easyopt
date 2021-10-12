use ndarray::prelude::*;
use ndarray::Zip;

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
pub struct Steffensen<'a> {
    f: Box<dyn 'a + Fn(&[f64]) -> Vec<f64>>,
    max_iter: usize,
}

impl<'a> Steffensen<'a> {
    pub fn new(f: impl 'a + Fn(&[f64]) -> Vec<f64>) -> Self {
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

    pub fn solve(&self, init: &[f64], atol: &[f64], rtol: &[f64]) -> Vec<f64> {
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
                    (x1 - x2).abs() < atol + rtol * x1.abs().max(x2.abs())
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
