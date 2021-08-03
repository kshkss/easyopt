use crate::traits::*;
use serde::Serialize;
use std::fs::File;
use std::io;

pub fn to_file<T>(filename: &str) -> io::Result<impl Monitor<T>>
where
    T: Report + Serialize,
{
    let f = File::create(filename)?;
    let f = io::BufWriter::new(f);
    let mut table = table_dump::Table::from_writer(f);
    let monitor = move |report: &T| table.serialize(&report);
    Ok(monitor)
}

pub fn to_stdout<T>() -> io::Result<impl Monitor<T>>
where
    T: Report + Serialize,
{
    let f = io::stdout();
    let f = io::BufWriter::new(f);
    let mut table = table_dump::Table::from_writer(f);
    let monitor = move |report: &T| table.serialize(&report);
    Ok(monitor)
}

pub fn to_stderr<T>() -> io::Result<impl Monitor<T>>
where
    T: Report + Serialize,
{
    let f = io::stderr();
    let f = io::BufWriter::new(f);
    let mut table = table_dump::Table::from_writer(f);
    let monitor = move |report: &T| table.serialize(&report);
    Ok(monitor)
}
