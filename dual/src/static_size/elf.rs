use num_traits::Float;
use crate::traits::ElementaryFunction;
use super::Dual;

impl<T, const N: usize> ElementaryFunction for Dual<T, N>
where
T: Float,
{
    fn negate(&self) -> Self {
        todo!()
    }

    fn recip(&self) -> Self {
        todo!()
    }

    fn sqrt(&self) -> Self{
        todo!()
    }
    fn cbrt(&self) -> Self{
        todo!()
    }
    fn powf(&self, n: Self) -> Self{
        todo!()
    }
    fn powi(&self, n: i32) -> Self{
        todo!()
    }

    fn exp(&self) -> Self{
        todo!()
    }
    fn exp2(&self) -> Self{
        todo!()
    }
    fn exp_m1(&self) -> Self{
        todo!()
    }

    fn ln(&self) -> Self{
        todo!()
    }
    fn log10(&self) -> Self{
        todo!()
    }
    fn log2(&self) -> Self{
        todo!()
    }
    fn log(&self, base: Self) -> Self{
        todo!()
    }
    fn ln_1p(&self) -> Self{
        todo!()
    }

    fn cos(&self) -> Self{
        todo!()
    }
    fn sin(&self) -> Self{
        todo!()
    }
    fn tan(&self) -> Self{
        todo!()
    }

    fn acos(&self) -> Self{
        todo!()
    }
    fn asin(&self) -> Self{
        todo!()
    }
    fn atan(&self) -> Self{
        todo!()
    }

    fn cosh(&self) -> Self{
        todo!()
    }
    fn sinh(&self) -> Self{
        todo!()
    }
    fn tanh(&self) -> Self{
        todo!()
    }

    fn acosh(&self) -> Self{
        todo!()
    }
    fn asinh(&self) -> Self{
        todo!()
    }
    fn atanh(&self) -> Self{
        todo!()
    }
}
