use crate::traits::*;

pub struct Executor<S, O> {
    solver: S,
    op: O,
}

pub struct ExecutorStage1<'a, S, O, P> {
    solver: S,
    op: O,
    report: P,
    monitor: Vec<Box<dyn 'a + Monitor<P>>>,
}

pub struct ExecutorReady<'a, S, O, P, F> {
    solver: S,
    op: O,
    report: P,
    monitor: Vec<Box<dyn 'a + Monitor<P>>>,
    criteria: F,
}

impl<S, O> Executor<S, O>
where
    S: Solver<O>,
{
    pub fn new(solver: S, op: O) -> Self {
        Self { solver, op }
    }

    pub fn report<'a, T>(self, report: T) -> ExecutorStage1<'a, S, O, T>
    where
        T: Report<Arg = S::ReportArg>,
    {
        ExecutorStage1::<'a, S, O, T> {
            solver: self.solver,
            op: self.op,
            report,
            monitor: Vec::with_capacity(4),
        }
    }
}

impl<'a, S, O, T> ExecutorStage1<'a, S, O, T>
where
    S: Solver<O>,
    T: Report<Arg = S::ReportArg>,
{
    pub fn terminate<F>(self, c: F) -> ExecutorReady<'a, S, O, T, F>
    where
        // F: Fn(&T) -> Result<(), f64>,
        F: Criteria<T>,
    {
        ExecutorReady::<'a, S, O, T, F> {
            solver: self.solver,
            op: self.op,
            report: self.report,
            monitor: self.monitor,
            criteria: c,
        }
    }

    pub fn add_monitor<'b, F>(mut self, f: F) -> Self
    where
        F: 'a + Monitor<T>,
    {
        self.monitor.push(Box::new(f));
        self
    }
}

impl<'a, S, O, T, F> ExecutorReady<'a, S, O, T, F>
where
    S: Solver<O>,
    T: Report<Arg = S::ReportArg>,
    F: Criteria<T>,
{
    pub fn run(&mut self, init: S::Variable) -> anyhow::Result<S::Variable> {
        let mut x = init;
        self.solver.init_report(&mut self.report, &x)?;
        for f in self.monitor.iter_mut() {
            f(&self.report)?;
        }
        let mut res = (self.criteria)(&self.report);
        while res.is_err() {
            x = self.solver.next_iter(&self.op, &x)?;
            self.solver.update_report(&mut self.report, &x)?;
            for f in self.monitor.iter_mut() {
                f(&self.report)?
            }
            res = (self.criteria)(&self.report);
        }
        Ok(x)
    }

    pub fn add_monitor<M>(mut self, f: M) -> Self
    where
        M: 'a + Monitor<T>,
    {
        self.monitor.push(Box::new(f));
        self
    }
}
