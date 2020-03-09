use crate::{Encoder, Decoder};

pub struct BinaryEncoder {
    data: Vec<u8>,
}

pub struct BinaryDecoder<'a> {
    data: &'a [u8],
    position: usize,
}

impl BinaryEncoder {
    pub fn new() -> BinaryEncoder {
        BinaryEncoder {
            data: Vec::new(),
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl<'a> BinaryDecoder<'a> {
    pub fn new(data: &'a [u8]) -> BinaryDecoder<'a> {
        BinaryDecoder {
            data,
            position: 0,
        }
    }
}

impl Encoder for BinaryEncoder {
    type Error = !;

    fn emit_bytes(&mut self, v: &[u8]) -> Result<(), Self::Error> {
        self.data.extend_from_slice(v);
        Ok(())
    }

    fn emit_unit(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn emit_bool(&mut self, v: bool) -> Result<(), Self::Error> {
        self.data.push(if v { 1 } else { 0 });
        Ok(())
    }

    fn emit_char(&mut self, v: char) -> Result<(), Self::Error> {
        self.data.extend((v as u32).to_ne_bytes().iter());
        Ok(())
    }

    fn emit_str(&mut self, v: &str) -> Result<(), Self::Error> {
        self.emit_usize(v.len())?;
        self.data.extend_from_slice(v.as_bytes());
        Ok(())
    }

    fn emit_u8(&mut self, v: u8) -> Result<(), Self::Error> {
        self.data.push(v);
        Ok(())
    }

    fn emit_u16(&mut self, v: u16) -> Result<(), Self::Error> {
        self.data.extend(v.to_ne_bytes().iter());
        Ok(())
    }

    fn emit_u32(&mut self, v: u32) -> Result<(), Self::Error> {
        self.data.extend(v.to_ne_bytes().iter());
        Ok(())
    }

    fn emit_u64(&mut self, v: u64) -> Result<(), Self::Error> {
        self.data.extend(v.to_ne_bytes().iter());
        Ok(())
    }

    fn emit_u128(&mut self, v: u128) -> Result<(), Self::Error> {
        self.data.extend(v.to_ne_bytes().iter());
        Ok(())
    }

    fn emit_usize(&mut self, v: usize) -> Result<(), Self::Error> {
        self.data.extend(v.to_ne_bytes().iter());
        Ok(())
    }

    fn emit_i8(&mut self, v: i8) -> Result<(), Self::Error> {
        self.data.push(unsafe { ::std::mem::transmute(v) });
        Ok(())
    }

    fn emit_i16(&mut self, v: i16) -> Result<(), Self::Error> {
        self.data.extend(v.to_ne_bytes().iter());
        Ok(())
    }

    fn emit_i32(&mut self, v: i32) -> Result<(), Self::Error> {
        self.data.extend(v.to_ne_bytes().iter());
        Ok(())
    }

    fn emit_i64(&mut self, v: i64) -> Result<(), Self::Error> {
        self.data.extend(v.to_ne_bytes().iter());
        Ok(())
    }

    fn emit_i128(&mut self, v: i128) -> Result<(), Self::Error> {
        self.data.extend(v.to_ne_bytes().iter());
        Ok(())
    }

    fn emit_isize(&mut self, v: isize) -> Result<(), Self::Error> {
        self.data.extend(v.to_ne_bytes().iter());
        Ok(())
    }

    fn emit_f32(&mut self, v: f32) -> Result<(), Self::Error> {
        self.data.extend(v.to_bits().to_ne_bytes().iter());
        Ok(())
    }

    fn emit_f64(&mut self, v: f64) -> Result<(), Self::Error> {
        self.data.extend(v.to_bits().to_ne_bytes().iter());
        Ok(())
    }
}

impl<'a> Decoder for BinaryDecoder<'a> {
    type Error = String;

    fn read_bytes<'b>(&'b mut self, n: usize) -> Result<&'b [u8], Self::Error> {
        Ok(&self.data[self.position..self.position + n])
    }

    fn read_unit(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn read_bool(&mut self) -> Result<bool, Self::Error> {
        self.position += 1;

        Ok(self.data[self.position - 1] != 0)
    }

    fn read_char(&mut self) -> Result<char, Self::Error> {
        let bits = self.read_u32()?;

        ::std::char::from_u32(bits)
            .ok_or(self.error("invalid char"))
    }

    fn read_str(&mut self) -> Result<::std::borrow::Cow<'_, str>, Self::Error> {
        let len = self.read_usize()?;
        let bytes = &self.data[self.position..self.position + len];
        let s = ::std::str::from_utf8(bytes).map_err(|e| e.to_string())?;

        self.position += len;

        Ok(::std::borrow::Cow::Borrowed(s))
    }

    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        self.position += 1;

        Ok(self.data[self.position - 1])
    }

    fn read_u16(&mut self) -> Result<u16, Self::Error> {
        let bytes = [
            self.data[self.position + 0],
            self.data[self.position + 1],
        ];

        self.position += 2;

        Ok(u16::from_ne_bytes(bytes))
    }

    fn read_u32(&mut self) -> Result<u32, Self::Error> {
        let bytes = [
            self.data[self.position + 0],
            self.data[self.position + 1],
            self.data[self.position + 2],
            self.data[self.position + 3],
        ];

        self.position += 4;

        Ok(u32::from_ne_bytes(bytes))
    }

    fn read_u64(&mut self) -> Result<u64, Self::Error> {
        let bytes = [
            self.data[self.position + 0],
            self.data[self.position + 1],
            self.data[self.position + 2],
            self.data[self.position + 3],
            self.data[self.position + 4],
            self.data[self.position + 5],
            self.data[self.position + 6],
            self.data[self.position + 7],
        ];

        self.position += 8;

        Ok(u64::from_ne_bytes(bytes))
    }

    fn read_u128(&mut self) -> Result<u128, Self::Error> {
        let bytes = [
            self.data[self.position + 0],
            self.data[self.position + 1],
            self.data[self.position + 2],
            self.data[self.position + 3],
            self.data[self.position + 4],
            self.data[self.position + 5],
            self.data[self.position + 6],
            self.data[self.position + 7],
            self.data[self.position + 8],
            self.data[self.position + 9],
            self.data[self.position + 10],
            self.data[self.position + 11],
            self.data[self.position + 12],
            self.data[self.position + 13],
            self.data[self.position + 14],
            self.data[self.position + 15],
        ];

        self.position += 16;

        Ok(u128::from_ne_bytes(bytes))
    }

    fn read_usize(&mut self) -> Result<usize, Self::Error> {
        match ::std::mem::size_of::<usize>() {
            1 => self.read_u8().map(|v| v as usize),
            2 => self.read_u16().map(|v| v as usize),
            4 => self.read_u32().map(|v| v as usize),
            8 => self.read_u64().map(|v| v as usize),
            16 => self.read_u128().map(|v| v as usize),
            _ => unreachable!(),
        }
    }

    fn read_i8(&mut self) -> Result<i8, Self::Error> {
        self.position += 1;

        Ok(unsafe { ::std::mem::transmute(self.data[self.position - 1]) })
    }

    fn read_i16(&mut self) -> Result<i16, Self::Error> {
        let bytes = [
            self.data[self.position + 0],
            self.data[self.position + 1],
        ];

        self.position += 2;

        Ok(i16::from_ne_bytes(bytes))
    }

    fn read_i32(&mut self) -> Result<i32, Self::Error> {
        let bytes = [
            self.data[self.position + 0],
            self.data[self.position + 1],
            self.data[self.position + 2],
            self.data[self.position + 3],
        ];

        self.position += 4;

        Ok(i32::from_ne_bytes(bytes))
    }

    fn read_i64(&mut self) -> Result<i64, Self::Error> {
        let bytes = [
            self.data[self.position + 0],
            self.data[self.position + 1],
            self.data[self.position + 2],
            self.data[self.position + 3],
            self.data[self.position + 4],
            self.data[self.position + 5],
            self.data[self.position + 6],
            self.data[self.position + 7],
        ];

        self.position += 8;

        Ok(i64::from_ne_bytes(bytes))
    }

    fn read_i128(&mut self) -> Result<i128, Self::Error> {
        let bytes = [
            self.data[self.position + 0],
            self.data[self.position + 1],
            self.data[self.position + 2],
            self.data[self.position + 3],
            self.data[self.position + 4],
            self.data[self.position + 5],
            self.data[self.position + 6],
            self.data[self.position + 7],
            self.data[self.position + 8],
            self.data[self.position + 9],
            self.data[self.position + 10],
            self.data[self.position + 11],
            self.data[self.position + 12],
            self.data[self.position + 13],
            self.data[self.position + 14],
            self.data[self.position + 15],
        ];

        self.position += 16;

        Ok(i128::from_ne_bytes(bytes))
    }

    fn read_isize(&mut self) -> Result<isize, Self::Error> {
        match ::std::mem::size_of::<isize>() {
            1 => self.read_i8().map(|v| v as isize),
            2 => self.read_i16().map(|v| v as isize),
            4 => self.read_i32().map(|v| v as isize),
            8 => self.read_i64().map(|v| v as isize),
            16 => self.read_i128().map(|v| v as isize),
            _ => unreachable!(),
        }
    }

    fn read_f32(&mut self) -> Result<f32, Self::Error> {
        let bits = self.read_u32()?;

        Ok(f32::from_bits(bits))
    }

    fn read_f64(&mut self) -> Result<f64, Self::Error> {
        let bits = self.read_u64()?;

        Ok(f64::from_bits(bits))
    }

    fn error(&mut self, msg: &str) -> Self::Error {
        msg.to_string()
    }
}
