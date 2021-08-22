use num_traits::{Float, FloatConst, FromPrimitive, NumOps, NumRef};
use std::ops::Neg;

macro_rules! from_elm {
    ($fname: ident) => {
        #[inline]
        fn $fname() -> Self {
            Self::from(<T as Float>::$fname())
        }
    };
}

macro_rules! check_by_elm {
    ($fname: ident) => {
        #[inline]
        fn $fname(&self) -> bool {
            <T as Float>::$fname(self.to_float())
        }
    };
}

pub trait FloatLike<T>: NumRef + NumOps<T> + Neg<Output=Self> + From<T>
where
    for<'r> &'r Self: NumOps<T, Self>,
    T: Float + FloatConst + FromPrimitive,
{
    fn to_float(&self) -> T;

    from_elm!(nan);
    from_elm!(infinity);
    from_elm!(neg_infinity);
    from_elm!(neg_zero);
    from_elm!(min_value);
    from_elm!(min_positive_value);
    from_elm!(max_value);

    check_by_elm!(is_nan);
    check_by_elm!(is_infinite);
    check_by_elm!(is_finite);
    check_by_elm!(is_normal);

    fn floor(&self) -> Self;
    fn ceil(&self) -> Self;
    fn round(&self) -> Self;
    fn trunc(&self) -> Self;
    fn fract(&self) -> Self;

    fn abs(&self) -> Self;
    fn signum(&self) -> Self;
    fn negate(&self) -> Self;
    check_by_elm!(is_sign_positive);
    check_by_elm!(is_sign_negative);

    fn mul_add(&self, a: &Self, b: &Self) -> Self;
    fn recip(&self) -> Self;
    fn powi(&self, n: i32) -> Self;
    fn powf(&self, n: &Self) -> Self;
    fn sqrt(&self) -> Self;
    fn cbrt(&self) -> Self;
    fn hypot(&self, other: &Self) -> Self;

    fn exp(&self) -> Self;
    fn exp2(&self) -> Self;
    fn exp_m1(&self) -> Self;

    fn ln(&self) -> Self;
    fn log10(&self) -> Self;
    fn log2(&self) -> Self;
    fn log(&self, base: &Self) -> Self;
    fn ln_1p(&self) -> Self;

    fn max(&self, other: &Self) -> Self;
    fn min(&self, other: &Self) -> Self;
    fn abs_sub(&self, other: &Self) -> Self;

    fn cos(&self) -> Self;
    fn sin(&self) -> Self;
    fn tan(&self) -> Self;

    fn acos(&self) -> Self;
    fn asin(&self) -> Self;
    fn atan(&self) -> Self;
    fn atan2(&self, other: &Self) -> Self;
    fn sin_cos(&self) -> (Self, Self);

    fn cosh(&self) -> Self;
    fn sinh(&self) -> Self;
    fn tanh(&self) -> Self;

    fn acosh(&self) -> Self;
    fn asinh(&self) -> Self;
    fn atanh(&self) -> Self;

    //fn integer_decode(&self) -> (u64, i62, i8);
    fn classify(&self) -> core::num::FpCategory {
        T::classify(self.to_float())
    }

    fn epsilon() -> T {
        T::epsilon()
    }

    fn to_degrees(&self) -> Self {
        let pi = T::PI();
        self * (T::from_i64(180).unwrap() / pi)
    }

    fn to_radians(&self) -> Self {
        let pi = T::PI();
        self * (pi / T::from_i64(180).unwrap())
    }
}

macro_rules! redirect {
    ($fname: ident $(, $arg: ident)*) => {
        #[inline]
        fn $fname(&self $(, $arg: &Self)*) -> Self {
            <Self as Float>::$fname(*self $(, *$arg)* )
        }
    };
}

macro_rules! impl_floatlike_for_float {
    ($primitive: ty) => {
        impl FloatLike<$primitive> for $primitive {
            #[inline]
            fn to_float(&self) -> Self {
                *self
            }

            #[inline]
            fn negate(&self) -> Self {
                -self
            }

            #[inline]
            fn powi(&self, n: i32) -> Self {
                <Self as Float>::powi(*self, n)
            }

            #[inline]
            fn sin_cos(&self) -> (Self, Self) {
                <Self as Float>::sin_cos(*self)
            }

            redirect!(floor);
            redirect!(ceil);
            redirect!(round);
            redirect!(trunc);
            redirect!(fract);
            redirect!(abs);
            redirect!(signum);
            redirect!(mul_add, a, b);
            redirect!(recip);
            redirect!(powf, a);
            redirect!(sqrt);
            redirect!(cbrt);
            redirect!(hypot, other);
            redirect!(exp);
            redirect!(exp2);
            redirect!(exp_m1);
            redirect!(ln);
            redirect!(log10);
            redirect!(log2);
            redirect!(log, a);
            redirect!(ln_1p);
            redirect!(max, other);
            redirect!(min, other);
            redirect!(abs_sub, other);
            redirect!(sin);
            redirect!(cos);
            redirect!(tan);
            redirect!(asin);
            redirect!(acos);
            redirect!(atan);
            redirect!(atan2, other2);
            redirect!(sinh);
            redirect!(cosh);
            redirect!(tanh);
            redirect!(asinh);
            redirect!(acosh);
            redirect!(atanh);
        }
    };
}

impl_floatlike_for_float!(f32);
impl_floatlike_for_float!(f64);
