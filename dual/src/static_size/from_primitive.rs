use super::Dual;
use num_traits::FromPrimitive;

impl<T, const N: usize> FromPrimitive for Dual<T, N>
where
    T: FromPrimitive + Copy,
{
    fn from_i64(n: i64) -> Option<Self> {
        Some(Self {
            x: T::from_i64(n)?,
            dx: [T::from_i64(0)?; N],
        })
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(Self {
            x: T::from_u64(n)?,
            dx: [T::from_u64(0)?; N],
        })
    }
}
