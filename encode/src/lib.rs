#![feature(never_type)]

pub mod binary;
mod impls;

pub use encode_derive::{Encode, Decode};
use std::borrow::Cow;

pub trait Encode {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error>;
}

pub trait Decode: Sized {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error>;
}

pub trait Encoder: Sized {
    type Error;

    fn emit_bytes(&mut self, v: &[u8]) -> Result<(), Self::Error>;
    fn emit_unit(&mut self) -> Result<(), Self::Error>;
    fn emit_bool(&mut self, v: bool) -> Result<(), Self::Error>;
    fn emit_char(&mut self, v: char) -> Result<(), Self::Error>;
    fn emit_str(&mut self, v: &str) -> Result<(), Self::Error>;
    fn emit_u8(&mut self, v: u8) -> Result<(), Self::Error>;
    fn emit_u16(&mut self, v: u16) -> Result<(), Self::Error>;
    fn emit_u32(&mut self, v: u32) -> Result<(), Self::Error>;
    fn emit_u64(&mut self, v: u64) -> Result<(), Self::Error>;
    fn emit_u128(&mut self, v: u128) -> Result<(), Self::Error>;
    fn emit_usize(&mut self, v: usize) -> Result<(), Self::Error>;
    fn emit_i8(&mut self, v: i8) -> Result<(), Self::Error>;
    fn emit_i16(&mut self, v: i16) -> Result<(), Self::Error>;
    fn emit_i32(&mut self, v: i32) -> Result<(), Self::Error>;
    fn emit_i64(&mut self, v: i64) -> Result<(), Self::Error>;
    fn emit_i128(&mut self, v: i128) -> Result<(), Self::Error>;
    fn emit_isize(&mut self, v: isize) -> Result<(), Self::Error>;
    fn emit_f32(&mut self, v: f32) -> Result<(), Self::Error>;
    fn emit_f64(&mut self, v: f64) -> Result<(), Self::Error>;

    /// # Warning
    /// This function expects v.size_hint() to be accurate
    fn emit_list<'a, T: Encode + 'a>(&mut self, v: impl IntoIterator<Item=&'a T>) -> Result<(), Self::Error> {
        let mut v = v.into_iter();

        self.emit_usize(v.size_hint().0)?;

        while let Some(item) = v.next() {
            <T as Encode>::encode(item, self)?;
        }

        Ok(())
    }

    /// # Warning
    /// This function expects v.size_hint() to be accurate
    fn emit_map<'a, K: Encode + 'a, V: Encode + 'a>(&mut self, v: impl IntoIterator<Item=(&'a K, &'a V)>) -> Result<(), Self::Error> {
        let mut v = v.into_iter();

        self.emit_usize(v.size_hint().0)?;

        while let Some((k, v)) = v.next() {
            <K as Encode>::encode(k, self)?;
            <V as Encode>::encode(v, self)?;
        }

        Ok(())
    }
}

pub trait Decoder: Sized {
    type Error;

    fn read_bytes<'a>(&'a mut self, n: usize) -> Result<&'a [u8], Self::Error>;
    fn read_unit(&mut self) -> Result<(), Self::Error>;
    fn read_bool(&mut self) -> Result<bool, Self::Error>;
    fn read_char(&mut self) -> Result<char, Self::Error>;
    fn read_str(&mut self) -> Result<Cow<'_, str>, Self::Error>;
    fn read_u8(&mut self) -> Result<u8, Self::Error>;
    fn read_u16(&mut self) -> Result<u16, Self::Error>;
    fn read_u32(&mut self) -> Result<u32, Self::Error>;
    fn read_u64(&mut self) -> Result<u64, Self::Error>;
    fn read_u128(&mut self) -> Result<u128, Self::Error>;
    fn read_usize(&mut self) -> Result<usize, Self::Error>;
    fn read_i8(&mut self) -> Result<i8, Self::Error>;
    fn read_i16(&mut self) -> Result<i16, Self::Error>;
    fn read_i32(&mut self) -> Result<i32, Self::Error>;
    fn read_i64(&mut self) -> Result<i64, Self::Error>;
    fn read_i128(&mut self) -> Result<i128, Self::Error>;
    fn read_isize(&mut self) -> Result<isize, Self::Error>;
    fn read_f32(&mut self) -> Result<f32, Self::Error>;
    fn read_f64(&mut self) -> Result<f64, Self::Error>;

    fn read_list<L: std::iter::FromIterator<T>, T: Decode>(&mut self) -> Result<L, Self::Error> {
        let len = self.read_usize()?;
        let mut items = Vec::new();

        for _ in 0..len {
            items.push(<T as Decode>::decode(self)?);
        }

        Ok(<L>::from_iter(items))
    }

    fn read_map<M: std::iter::FromIterator<(K, V)>, K: Decode, V: Decode>(&mut self) -> Result<M, Self::Error> {
        let len = self.read_usize()?;
        let mut items = Vec::new();

        for _ in 0..len {
            let k = <K as Decode>::decode(self)?;
            let v = <V as Decode>::decode(self)?;

            items.push((k, v));
        }

        Ok(<M>::from_iter(items))
    }

    fn error(&mut self, msg: &str) -> Self::Error;
}
