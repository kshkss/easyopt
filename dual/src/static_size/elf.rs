use super::Dual;
use crate::traits::ElementaryFunction;
use num_traits::{Float, FromPrimitive, NumAssign};

macro_rules! unary {
    ($self:ident, $name:ident, $fx:expr, $dfdx:expr) => {
        let mut $name = Self {
            x: $fx,
            ..$self.clone()
        };
        let derivative = $dfdx;
        for dx in $name.dx.iter_mut() {
            *dx *= derivative;
        }
        return $name
    };
}

macro_rules! binary {
    ($self:ident, $rhs:ident, $name:ident, $fx:expr, $dfdx:expr, $dfdy: expr) => {
        let mut $name = Self {
            x: $fx,
            ..$self.clone()
        };
        let a = $dfdx;
        let b = $dfdy;
        for (dx, dx2) in $name.dx.iter_mut().zip($rhs.dx.iter()) {
            *dx = dx.mul_add(a, b * *dx2);
        }
        return $name
    };
}

impl<T, const N: usize> ElementaryFunction for Dual<T, N>
where
    T: Float + NumAssign + FromPrimitive,
{
    fn negate(&self) -> Self {
        unary!(self, new, -self.x, T::from_i32(-1).unwrap());
    }

    fn recip(&self) -> Self {
        unary!(self, new, self.x.recip(), -self.x.powi(-2));
    }

    fn sqrt(&self) -> Self {
        unary!(
            self,
            new,
            self.x.sqrt(),
            (T::from_i32(2).unwrap() * new.x).recip()
        );
    }
    fn cbrt(&self) -> Self {
        unary!(
            self,
            new,
            self.x.cbrt(),
            (T::from_i32(3).unwrap() * new.x.powi(2)).recip()
        );
    }
    fn powi(&self, n: i32) -> Self {
        unary!(
            self,
            new,
            self.x.powi(n),
            T::from_i32(n).unwrap() * self.x.powi(n - 1)
        );
    }
    fn powf(&self, n: &Self) -> Self {
        binary!(
            self,
            n,
            new,
            self.x.powf(n.x),
            new.x * n.x / self.x,
            new.x * self.x.ln()
        );
    }

    fn exp(&self) -> Self {
        unary!(self, new, self.x.exp(), new.x);
    }

    fn exp2(&self) -> Self {
        unary!(
            self,
            new,
            self.x.exp2(),
            new.x * T::from_i32(2).unwrap().ln()
        );
    }

    fn exp_m1(&self) -> Self {
        unary!(self, new, self.x.exp_m1(), self.x.exp());
    }

    fn ln(&self) -> Self {
        unary!(self, new, self.x.ln(), self.x.recip());
    }
    fn log10(&self) -> Self {
        unary!(
            self,
            new,
            self.x.log10(),
            (T::from_i32(10).unwrap().ln() * self.x).recip()
        );
    }
    fn log2(&self) -> Self {
        unary!(
            self,
            new,
            self.x.log2(),
            (T::from_i32(2).unwrap().ln() * self.x).recip()
        );
    }
    fn log(&self, base: &Self) -> Self {
        binary!(
            self,
            base,
            new,
            self.x.log(base.x),
            (self.x * base.x.ln()).recip(),
            -self.x.ln() / (base.x * base.x.ln().powi(2))
        );
    }
    fn ln_1p(&self) -> Self {
        unary!(self, new, self.x.ln_1p(), (T::one() + self.x).recip());
    }

    fn cos(&self) -> Self {
        unary!(self, new, self.x.cos(), -self.x.sin());
    }
    fn sin(&self) -> Self {
        unary!(self, new, self.x.sin(), self.x.cos());
    }
    fn tan(&self) -> Self {
        unary!(self, new, self.x.tan(), T::one() + new.x.powi(2));
    }

    fn acos(&self) -> Self {
        unary!(self, new, self.x.acos(), -new.x.sin().recip());
    }
    fn asin(&self) -> Self {
        unary!(self, new, self.x.asin(), new.x.cos().recip());
    }
    fn atan(&self) -> Self {
        unary!(
            self,
            new,
            self.x.atan(),
            (T::one() + self.x.powi(2)).recip()
        );
    }

    fn cosh(&self) -> Self {
        unary!(self, new, self.x.cosh(), self.x.sinh());
    }
    fn sinh(&self) -> Self {
        unary!(self, new, self.x.sinh(), self.x.cosh());
    }
    fn tanh(&self) -> Self {
        unary!(self, new, self.x.tanh(), T::one() - new.x.powi(2));
    }

    fn acosh(&self) -> Self {
        unary!(self, new, self.x.acosh(), new.x.sinh().recip());
    }
    fn asinh(&self) -> Self {
        unary!(self, new, self.x.asinh(), new.x.cosh().recip());
    }
    fn atanh(&self) -> Self {
        unary!(
            self,
            new,
            self.x.atan(),
            (T::one() - self.x.powi(2)).recip()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    macro_rules! assert_id {
        ($e1: expr, $e2: expr $(, $opt:ident = $val:expr)*) => {
            let a = $e1;
            let b = $e2;
            assert_relative_eq!(*a.val(), *b.val() $(, $opt = $val)*);
            for (&dx1, &dx2) in a.grad().iter().zip(b.grad().iter()) {
                assert_relative_eq!(dx1, dx2 $(, $opt = $val)* );
            }
        };
    }

    #[test]
    fn negate() -> anyhow::Result<()> {
        let x = Dual::<f64, 1>::new(0, 3.0).negate();
        assert_relative_eq!(-3.0, *x.val());
        assert_relative_eq!(-1.0, x.grad()[0]);
        Ok(())
    }

    #[test]
    fn recip() -> anyhow::Result<()> {
        let x = Dual::<f64, 1>::new(0, 3.0).recip();
        assert_relative_eq!(1.0 / 3.0, *x.val());
        assert_relative_eq!(-1.0 / 9.0, x.grad()[0]);

        assert_id!(x.recip() * &x, Dual::<f64, 1>::constant(1.));
        assert_id!(x.recip().recip(), &x);
        assert_id!(&x / x.recip(), &x * &x);
        Ok(())
    }

    #[test]
    fn sqrt() -> anyhow::Result<()> {
        let x = Dual::<f64, 1>::new(0, 3.0).sqrt();
        assert_relative_eq!(f64::sqrt(3.0), *x.val());
        assert_relative_eq!(0.5 / f64::sqrt(3.0), x.grad()[0]);

        assert_id!(x.sqrt() * x.sqrt(), &x);
        assert_id!(x.sqrt() / &x, x.sqrt().recip());
        Ok(())
    }

    #[test]
    fn powi() -> anyhow::Result<()> {
        let x = Dual::<f64, 2>::new(0, 0.5);
        assert_id!(x.powi(2) * x.powi(3), x.powi(5));
        assert_id!(x.powi(2) * x.powi(-3), x.powi(-1));
        assert_id!(x.powi(2) / x.powi(3), x.powi(-1));
        assert_id!(x.powi(2) / x.powi(-3), x.powi(5));
        assert_id!(x.sqrt().powi(2), &x);
        assert_id!(x.powi(2).sqrt(), &x);
        assert_id!(x.recip().powi(-1), &x);
        assert_id!(x.powi(-1).recip(), &x);

        Ok(())
    }

    #[test]
    fn cbrt() -> anyhow::Result<()> {
        let x = Dual::<f64, 2>::new(0, 0.5);
        assert_id!(x.cbrt().powi(3), &x);
        assert_id!(x.powi(3).cbrt(), &x);
        assert_id!(x.cbrt().powi(2), &x / x.cbrt());
        assert_id!(x.cbrt().powi(2), &x * x.cbrt().recip());

        Ok(())
    }

    #[test]
    fn powf() -> anyhow::Result<()> {
        let x = Dual::<f64, 2>::new(0, f64::exp(1.)).powf(&Dual::<f64, 2>::new(1, 2.0));
        assert_relative_eq!(f64::exp(2.), *x.val());
        assert_relative_eq!(2. * f64::exp(1.), x.grad()[0]);
        assert_relative_eq!(f64::exp(2.), x.grad()[1]);

        let y = Dual::<f64, 1>::new(0, f64::exp(1.)).powf(&Dual::<f64, 1>::new(0, 2.0));
        assert_relative_eq!(y.grad()[0], x.grad()[0] + x.grad()[1]);

        let x = Dual::<f64, 2>::new(0, 2.);
        let z = Dual::<f64, 2>::new(1, f64::exp(1.));
        assert_id!(x.powf(&z.negate()), x.powf(&z).recip());
        assert_id!(
            x.powf(&z) * x.powf(&z.negate()),
            Dual::<f64, 2>::constant(1.)
        );
        assert_id!(x.powf(&z) * x.powf(&z), x.powf(&(&z + &z)));
        assert_id!(x.powf(&z).powf(&z), x.powf(&(&z * &z)));
        assert_id!(x.sqrt() * x.cbrt(), x.powf(&Dual::<f64, 2>::constant(5./6.)), epsilon = f64::EPSILON * 10.);

        Ok(())
    }

    #[test]
    fn exp() -> anyhow::Result<()> {
        let x = Dual::<f64, 2>::new(0, 0.5);
        let y = Dual::<f64, 2>::new(1, 1.5);
        assert_id!((&x + &y).exp(), x.exp() * y.exp());
        assert_id!((&x * &y).exp(), x.exp().powf(&y));

        Ok(())
    }

    #[test]
    fn exp2() -> anyhow::Result<()> {
        let x = Dual::<f64, 2>::new(0, 0.5);
        let y = Dual::<f64, 2>::new(1, 1.5);
        assert_id!((&x + &y).exp2(), x.exp2() * y.exp2());
        assert_id!((&x * &y).exp2(), x.exp2().powf(&y));

        Ok(())
    }

    #[test]
    fn ln() -> anyhow::Result<()> {
        let x = Dual::<f64, 2>::new(0, 0.5);
        let y = Dual::<f64, 2>::new(1, 1.5);
        assert_id!(x.ln().exp(), &x);
        assert_id!(x.exp().ln(), &x);
        assert_id!((&x * &y).ln(), x.ln() + y.ln());
        assert_id!(&y * x.ln(), x.powf(&y).ln());
        assert_id!(x.sqrt().ln(), x.ln() * 0.5);
        assert_id!(x.cbrt().ln(), x.ln() / 3.);

        Ok(())
    }

    #[test]
    fn log2() -> anyhow::Result<()> {
        let x = Dual::<f64, 2>::new(0, 0.5);
        let y = Dual::<f64, 2>::new(1, 1.5);
        assert_id!(x.log2().exp2(), &x);
        assert_id!(x.exp2().log2(), &x);
        assert_id!((&x * &y).log2(), x.log2() + y.log2());
        assert_id!(&y * x.log2(), x.powf(&y).log2());

        Ok(())
    }

    #[test]
    fn log10() -> anyhow::Result<()> {
        let x = Dual::<f64, 2>::new(0, 0.5);
        let y = Dual::<f64, 2>::new(1, 1.5);
        assert_id!((&x * &y).log10(), x.log10() + y.log10());
        assert_id!(&y * x.log10(), x.powf(&y).log10());

        Ok(())
    }

    #[test]
    fn log() -> anyhow::Result<()> {
        let x = Dual::<f64, 2>::new(0, 0.5);
        let y = Dual::<f64, 2>::new(1, 1.5);
        assert_id!(x.ln() / y.ln(), x.log(&y));

        Ok(())
    }

    #[test]
    fn sin_cos() -> anyhow::Result<()> {
        let x = Dual::<f64, 1>::new(0, 0.5);
        assert_id!(
            x.sin().powi(2) + x.cos() * x.cos(),
            Dual::<f64, 1>::constant(1.)
        );
        assert_id!((&x * 2.).sin(), x.sin() * x.cos() * 2.);
        assert_id!((&x * 2.).cos(), x.cos().powi(2) - x.sin().powi(2));
        assert_id!((&x * 0.5).sin().powi(2), (x.cos().negate() + 1.) * 0.5);
        assert_id!((&x * 0.5).cos().powi(2), (x.cos() + 1.) * 0.5);

        Ok(())
    }

    #[test]
    fn tan() -> anyhow::Result<()> {
        let x = Dual::<f64, 1>::new(0, 0.5);
        assert_id!(x.tan(), x.sin() / x.cos());

        Ok(())
    }

    #[test]
    fn asin() -> anyhow::Result<()> {
        let x = Dual::<f64, 1>::new(0, 0.5).asin();
        assert_relative_eq!(f64::asin(0.5), *x.val());
        assert_relative_eq!(1. / (1. - 0.5 * 0.5).sqrt(), x.grad()[0]);

        assert_id!(x.sin().asin(), &x);
        assert_id!(x.asin().sin(), &x);
        Ok(())
    }

    #[test]
    fn acos() -> anyhow::Result<()> {
        let x = Dual::<f64, 1>::new(0, 0.5).asin();

        assert_id!(x.cos().acos(), &x);
        assert_id!(x.acos().cos(), &x);
        Ok(())
    }

    #[test]
    fn sinh_cosh() -> anyhow::Result<()> {
        let x = Dual::<f64, 1>::new(0, 0.5);
        assert_id!(
            x.cosh().powi(2) - x.sinh() * x.sinh(),
            Dual::<f64, 1>::constant(1.)
        );
        assert_id!(x.sinh(), (x.exp() - x.negate().exp()) * 0.5);
        assert_id!(x.cosh(), (x.exp() + x.negate().exp()) * 0.5);
        Ok(())
    }

    #[test]
    fn tanh() -> anyhow::Result<()> {
        let x = Dual::<f64, 1>::new(0, 0.5);
        assert_id!(x.tanh(), x.sinh() / x.cosh());

        Ok(())
    }
}
