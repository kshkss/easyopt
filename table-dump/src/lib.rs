use serde::ser;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    //#[error("Too complicated data structure to dump as a table. Try a flattened structure.")]
    #[error("Map and variant is not acceptable except unit-variant pattern.")]
    TooComplicated,
    #[error("failed with serializing data: {0}")]
    Failure(String),
}

type Result<T> = std::result::Result<T, Error>;

impl ser::Error for Error {
    #[cold]
    fn custom<T: std::fmt::Display>(msg: T) -> Error {
        Error::Failure(msg.to_string())
    }
}

pub mod table;
use table::Serializer;

pub struct Table<W> {
    writer: W,
    columns: Option<Vec<String>>,
}

use std::io;
impl<W: io::Write> Table<W> {
    pub fn from_writer(writer: W) -> Self {
        Self {
            writer,
            columns: None,
        }
    }

    pub fn columns(&mut self, cols: Vec<String>) {
        self.columns = Some(cols);
    }

    pub fn serialize<T: ser::Serialize>(&mut self, value: &T) -> anyhow::Result<()> {
        let mut ser = Serializer::new();
        value.serialize(&mut ser)?;
        match self
            .columns
            .as_ref()
            .map(|columns| columns.len() == ser.columns.len())
        {
            None => {
                write!(self.writer, "{}", ser.columns[0])?;
                for col in ser.columns.iter().skip(1) {
                    write!(self.writer, "\t{}", col)?;
                }
                write!(self.writer, "\n{}", ser.values[0])?;
                for val in ser.values.iter().skip(1) {
                    write!(self.writer, "\t{}", val)?;
                }
                self.columns.replace(ser.columns);
            }
            Some(p) => {
                if p {
                    write!(self.writer, "\n{}", ser.values[0])?;
                    for val in ser.values.iter().skip(1) {
                        write!(self.writer, "\t{}", val)?;
                    }
                } else {
                    return Err(Error::Failure(String::from("Data structure was changed")).into())
                }
            }
        }
        self.writer.flush()?;
        Ok(())
    }
}
