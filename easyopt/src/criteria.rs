use crate::traits::*;

pub fn when<R: Report>(f: impl Fn(&R) -> bool) -> impl for<'a> Fn(&'a R) -> Result<(), f64> {
    move |report| {
        if f(report) {
            Ok(())
        } else {
            Err(0.)
        }
    }
}
