use crate::{Error, Result};
use serde::{ser, Serialize};

#[derive(Debug, Clone)]
enum Key {
    Label(String),
    Index(usize),
}

impl std::string::ToString for Key {
    fn to_string(&self) -> String {
        use Key::*;
        match self {
            Label(name) => name.clone(),
            Index(i) => format!("[{}]", i),
        }
    }
}

pub struct Serializer {
    key_stack: Vec<Key>,
    pub columns: Vec<String>,
    pub values: Vec<String>,
}

impl Serializer {
    pub fn new() -> Self {
        Self {
            key_stack: Vec::new(),
            columns: Vec::new(),
            values: Vec::new(),
        }
    }
}

impl Default for Serializer {
    fn default() -> Self {
        Self::new()
    }
}

fn estimate_buffer_size(keys: &[Key]) -> usize {
    use Key::*;
    let size = keys.iter().fold(0, |acc, key| match key {
        Label(v) => acc + v.len(),
        Index(_) => acc + 6,
    });
    size + keys.len()
}

fn concat_name(keys: &[Key]) -> String {
    use Key::*;
    if keys.is_empty() {
        String::from("")
    } else if keys.len() == 1 {
        keys[0].to_string()
    } else {
        let size = estimate_buffer_size(keys);
        let mut buf = String::with_capacity(size);
        let mut head = true;
        for key in keys.iter() {
            match key {
                Label(v) => {
                    if head {
                        head = false;
                    } else {
                        buf.push('.');
                    }
                    buf.push_str(v)
                }
                Index(i) => buf.push_str(&format!("[{}]", i)),
            }
        }
        buf
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        if v {
            self.serialize_u64(1)
        } else {
            self.serialize_u64(0)
        }
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.columns.push(concat_name(&self.key_stack));
        self.values.push(v.to_string());
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.columns.push(concat_name(&self.key_stack));
        self.values.push(v.to_string());
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.columns.push(concat_name(&self.key_stack));
        self.values.push(v.to_string());
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.columns.push(concat_name(&self.key_stack));
        self.values.push(String::from(v));
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.columns.push(concat_name(&self.key_stack));
        self.values.push(base64::encode(v));
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        self.columns.push(concat_name(&self.key_stack));
        self.values.push(String::from("null"));
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)?;
        Ok(())
    }

    fn serialize_unit(self) -> Result<()> {
        self.columns.push(concat_name(&self.key_stack));
        self.values.push(String::from("null"));
        Ok(())
    }

    // Unit struct means a named value containing no data.
    fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
        self.columns.push(concat_name(&self.key_stack));
        self.values.push(String::from(name));
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _varint_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.columns.push(concat_name(&self.key_stack));
        self.values.push(String::from(variant));
        Ok(())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::TooComplicated)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.key_stack.push(Key::Index(0));
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        self.key_stack.push(Key::Index(0));
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.key_stack.push(Key::Index(0));
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::TooComplicated)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::TooComplicated)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::TooComplicated)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;
        if let Some(Key::Index(i)) = self.key_stack.pop() {
            self.key_stack.push(Key::Index(i + 1));
        } else {
            unreachable!();
        }
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.key_stack.pop();
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;
        if let Some(Key::Index(i)) = self.key_stack.pop() {
            self.key_stack.push(Key::Index(i + 1));
        } else {
            unreachable!();
        }
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.key_stack.pop();
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;
        if let Some(Key::Index(i)) = self.key_stack.pop() {
            self.key_stack.push(Key::Index(i + 1));
        } else {
            unreachable!();
        }
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.key_stack.pop();
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<()> {
        unreachable!()
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<()> {
        unreachable!()
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.key_stack.push(Key::Label(String::from(key)));
        value.serialize(&mut **self)?;
        self.key_stack.pop();
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unreachable!()
    }

    fn end(self) -> Result<()> {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i64() -> Result<()> {
        let v: i64 = -1;
        let mut ser = Serializer::new();
        v.serialize(&mut ser)?;
        assert_eq!(vec![""], ser.columns);
        assert_eq!(vec!["-1"], ser.values);
        Ok(())
    }

    #[test]
    fn test_u64() -> Result<()> {
        let v: u64 = 1;
        let mut ser = Serializer::new();
        v.serialize(&mut ser)?;
        assert_eq!(vec![""], ser.columns);
        assert_eq!(vec!["1"], ser.values);
        Ok(())
    }

    #[test]
    fn test_f64_0() -> Result<()> {
        let v: f64 = 1.;
        let mut ser = Serializer::new();
        v.serialize(&mut ser)?;
        assert_eq!(vec![""], ser.columns);
        assert_eq!(vec!["1"], ser.values);
        Ok(())
    }

    #[test]
    fn test_f64_1() -> Result<()> {
        let v: f64 = std::f64::NEG_INFINITY;
        let mut ser = Serializer::new();
        v.serialize(&mut ser)?;
        assert_eq!(vec![""], ser.columns);
        assert_eq!(vec!["-inf"], ser.values);
        Ok(())
    }

    #[test]
    fn test_f64_2() -> Result<()> {
        let v: f64 = std::f64::INFINITY;
        let mut ser = Serializer::new();
        v.serialize(&mut ser)?;
        assert_eq!(vec![""], ser.columns);
        assert_eq!(vec!["inf"], ser.values);
        Ok(())
    }

    #[test]
    fn test_f64_3() -> Result<()> {
        let v: f64 = std::f64::NAN;
        let mut ser = Serializer::new();
        v.serialize(&mut ser)?;
        assert_eq!(vec![""], ser.columns);
        assert_eq!(vec!["NaN"], ser.values);
        Ok(())
    }

    #[test]
    fn test_true() -> Result<()> {
        let v = true;
        let mut ser = Serializer::new();
        v.serialize(&mut ser)?;
        assert_eq!(vec![""], ser.columns);
        assert_eq!(vec!["1"], ser.values);
        Ok(())
    }

    #[test]
    fn test_false() -> Result<()> {
        let v = false;
        let mut ser = Serializer::new();
        v.serialize(&mut ser)?;
        assert_eq!(vec![""], ser.columns);
        assert_eq!(vec!["0"], ser.values);
        Ok(())
    }

    #[test]
    fn test_str() -> Result<()> {
        let v = "abs";
        let mut ser = Serializer::new();
        v.serialize(&mut ser)?;
        assert_eq!(vec![""], ser.columns);
        assert_eq!(vec!["abs"], ser.values);
        Ok(())
    }

    #[test]
    fn test_bytes() -> Result<()> {
        let v = serde_bytes::ByteBuf::from(vec![1, 2, 123]);
        let mut ser = Serializer::new();
        v.serialize(&mut ser)?;
        assert_eq!(vec![""], ser.columns);
        assert_eq!(vec!["AQJ7"], ser.values);
        Ok(())
    }

    #[test]
    fn test_unit() -> Result<()> {
        let v = ();
        let mut ser = Serializer::new();
        v.serialize(&mut ser)?;
        assert_eq!(vec![""], ser.columns);
        assert_eq!(vec!["null"], ser.values);
        Ok(())
    }

    #[test]
    fn test_none() -> Result<()> {
        let v: Option<usize> = None;
        let mut ser = Serializer::new();
        v.serialize(&mut ser)?;
        assert_eq!(vec![""], ser.columns);
        assert_eq!(vec!["null"], ser.values);
        Ok(())
    }

    #[test]
    fn test_some() -> Result<()> {
        let v = Some(42);
        let mut ser = Serializer::new();
        v.serialize(&mut ser)?;
        assert_eq!(vec![""], ser.columns);
        assert_eq!(vec!["42"], ser.values);
        Ok(())
    }

    #[test]
    fn test_tuple() -> Result<()> {
        let v = ("a", 2, -3.);
        let mut ser = Serializer::new();
        v.serialize(&mut ser)?;
        assert_eq!(vec!["[0]", "[1]", "[2]"], ser.columns);
        assert_eq!(vec!["a", "2", "-3"], ser.values);
        Ok(())
    }

    #[test]
    fn test_seq() -> Result<()> {
        let v: Vec<f64> = vec![1., 2., 3.];
        let mut ser = Serializer::new();
        v.serialize(&mut ser)?;
        assert_eq!(vec!["[0]", "[1]", "[2]"], ser.columns);
        assert_eq!(vec!["1", "2", "3"], ser.values);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "TooComplicated")]
    fn test_map() {
        use std::collections::HashMap;
        let mut v = HashMap::<String, usize>::new();
        v.insert(String::from("a"), 10);
        v.insert(String::from("b"), 2);
        let mut ser = Serializer::new();
        v.serialize(&mut ser).unwrap();
    }

    #[derive(Serialize)]
    enum Bar {
        A,                      // serialize_unit_variant
        B(&'static str),        // serialize_newtype_variant
        C(usize, f32),          // serialize_tuple_variant
        D { a: f64, b: usize }, // serialize_struct_variant
    }

    #[test]
    fn test_unit_variant() -> Result<()> {
        let mut ser = Serializer::new();
        let v = Bar::A;
        v.serialize(&mut ser)?;
        assert_eq!(vec![""], ser.columns);
        assert_eq!(vec!["A"], ser.values);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "TooComplicated")]
    fn test_newtype_variant() {
        let mut ser = Serializer::new();
        let v = Bar::B("abc");
        v.serialize(&mut ser).unwrap();
    }

    #[test]
    #[should_panic(expected = "TooComplicated")]
    fn test_tuple_variant() {
        let mut ser = Serializer::new();
        let v = Bar::C(1, 3.);
        v.serialize(&mut ser).unwrap();
    }

    #[test]
    #[should_panic(expected = "TooComplicated")]
    fn test_struct_variant() {
        let mut ser = Serializer::new();
        let v = Bar::D { a: 1., b: 32 };
        v.serialize(&mut ser).unwrap();
    }

    #[derive(Serialize)]
    struct OStruct {
        a: f64,
        b: usize,
    }

    #[test]
    fn test_struct() -> Result<()> {
        let mut ser = Serializer::new();
        let v = OStruct { a: 1., b: 0 };
        v.serialize(&mut ser)?;
        assert_eq!(vec!["a", "b"], ser.columns);
        assert_eq!(vec!["1", "0"], ser.values);
        Ok(())
    }

    #[derive(Serialize)]
    struct NStruct(f64);

    #[test]
    fn test_newtype_struct() -> Result<()> {
        let mut ser = Serializer::new();
        let v = NStruct(3.);
        v.serialize(&mut ser)?;
        assert_eq!(vec![""], ser.columns);
        assert_eq!(vec!["3"], ser.values);
        Ok(())
    }

    #[derive(Serialize)]
    struct TStruct(usize, f64); // serialize_tuple_struct

    #[test]
    fn test_tuple_struct() -> Result<()> {
        let mut ser = Serializer::new();
        let v = TStruct(1, 3.);
        v.serialize(&mut ser)?;
        assert_eq!(vec!["[0]", "[1]"], ser.columns);
        assert_eq!(vec!["1", "3"], ser.values);
        Ok(())
    }

    #[derive(Serialize)]
    struct UStruct; // serialize_unit_struct

    #[test]
    fn test_unit_struct() -> Result<()> {
        let mut ser = Serializer::new();
        let v = UStruct;
        v.serialize(&mut ser)?;
        assert_eq!(vec![""], ser.columns);
        assert_eq!(vec!["UStruct"], ser.values);
        Ok(())
    }

    use ndarray::prelude::*;
    #[derive(Serialize)]
    struct TestStruct03 {
        n: usize,
        a: Array2<f64>,
    }
    #[test]
    fn test_ndarray() -> Result<()> {
        let mut ser = Serializer::new();
        let v = TestStruct03 {
            n: 100,
            a: array![[1., 2.], [3., 4.]],
        };
        v.serialize(&mut ser)?;
        assert_eq!(
            vec![
                "n",
                "a.v",
                "a.dim[0]",
                "a.dim[1]",
                "a.data[0]",
                "a.data[1]",
                "a.data[2]",
                "a.data[3]"
            ],
            ser.columns
        );
        assert_eq!(vec!["100", "1", "2", "2", "1", "2", "3", "4"], ser.values);
        Ok(())
    }
}
