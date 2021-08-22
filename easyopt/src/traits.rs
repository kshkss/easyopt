use crate::error::*;

pub trait Solver<T> {
    type Variable;
    type ReportArg;
    fn next_iter(&mut self, op: &T, x: &Self::Variable) -> Result<Self::Variable, Error>;
    fn init_report<R: Report<Arg = Self::ReportArg, Op = T>>(
        &self,
        report: &mut R,
        op: &T,
        x: &Self::Variable,
    ) -> Result<(), Error>;
    fn update_report<R: Report<Arg = Self::ReportArg, Op = T>>(
        &self,
        report: &mut R,
        op: &T,
        x: &Self::Variable,
    ) -> Result<(), Error>;
}

pub trait Report {
    type Arg;
    type Op;
    fn init(&mut self, op: &Self::Op, s: &Self::Arg) -> Result<(), Error>;
    fn update(&mut self, op: &Self::Op, s: &Self::Arg) -> Result<(), Error>;
}

pub trait Monitor<T>: FnMut(&T) -> anyhow::Result<()> {}
impl<T: Report, F> Monitor<T> for F where F: FnMut(&T) -> anyhow::Result<()> {}

pub trait Criteria<T>: Fn(&T) -> Result<(), f64> {}
impl<T: Report, F> Criteria<T> for F where F: Fn(&T) -> Result<(), f64> {}
