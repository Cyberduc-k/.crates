use crate::{Encode, Decode, Encoder, Decoder};

impl Encode for () {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_unit()
    }
}

impl Encode for bool {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_bool(*self)
    }
}

impl Encode for char {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_char(*self)
    }
}

impl Encode for str {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_str(self)
    }
}

impl Encode for String {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_str(self)
    }
}

impl Encode for u8 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_u8(*self)
    }
}

impl Encode for u16 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_u16(*self)
    }
}

impl Encode for u32 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_u32(*self)
    }
}

impl Encode for u64 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_u64(*self)
    }
}

impl Encode for u128 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_u128(*self)
    }
}

impl Encode for usize {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_usize(*self)
    }
}

impl Encode for i8 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_i8(*self)
    }
}

impl Encode for i16 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_i16(*self)
    }
}

impl Encode for i32 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_i32(*self)
    }
}

impl Encode for i64 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_i64(*self)
    }
}

impl Encode for i128 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_i128(*self)
    }
}

impl Encode for isize {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_isize(*self)
    }
}

impl Encode for f32 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_f32(*self)
    }
}

impl Encode for f64 {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_f64(*self)
    }
}

impl Decode for () {
    fn decode<D: Decoder>(d: &mut D) -> Result<(), D::Error> {
        d.read_unit()
    }
}

impl Decode for bool {
    fn decode<D: Decoder>(d: &mut D) -> Result<bool, D::Error> {
        d.read_bool()
    }
}

impl Decode for char {
    fn decode<D: Decoder>(d: &mut D) -> Result<char, D::Error> {
        d.read_char()
    }
}

impl Decode for String {
    fn decode<D: Decoder>(d: &mut D) -> Result<String, D::Error> {
        d.read_str().map(|r| r.into_owned())
    }
}

impl Decode for u8 {
    fn decode<D: Decoder>(d: &mut D) -> Result<u8, D::Error> {
        d.read_u8()
    }
}

impl Decode for u16 {
    fn decode<D: Decoder>(d: &mut D) -> Result<u16, D::Error> {
        d.read_u16()
    }
}

impl Decode for u32 {
    fn decode<D: Decoder>(d: &mut D) -> Result<u32, D::Error> {
        d.read_u32()
    }
}

impl Decode for u64 {
    fn decode<D: Decoder>(d: &mut D) -> Result<u64, D::Error> {
        d.read_u64()
    }
}

impl Decode for u128 {
    fn decode<D: Decoder>(d: &mut D) -> Result<u128, D::Error> {
        d.read_u128()
    }
}

impl Decode for usize {
    fn decode<D: Decoder>(d: &mut D) -> Result<usize, D::Error> {
        d.read_usize()
    }
}

impl Decode for i8 {
    fn decode<D: Decoder>(d: &mut D) -> Result<i8, D::Error> {
        d.read_i8()
    }
}

impl Decode for i16 {
    fn decode<D: Decoder>(d: &mut D) -> Result<i16, D::Error> {
        d.read_i16()
    }
}

impl Decode for i32 {
    fn decode<D: Decoder>(d: &mut D) -> Result<i32, D::Error> {
        d.read_i32()
    }
}

impl Decode for i64 {
    fn decode<D: Decoder>(d: &mut D) -> Result<i64, D::Error> {
        d.read_i64()
    }
}

impl Decode for i128 {
    fn decode<D: Decoder>(d: &mut D) -> Result<i128, D::Error> {
        d.read_i128()
    }
}

impl Decode for isize {
    fn decode<D: Decoder>(d: &mut D) -> Result<isize, D::Error> {
        d.read_isize()
    }
}

impl Decode for f32 {
    fn decode<D: Decoder>(d: &mut D) -> Result<f32, D::Error> {
        d.read_f32()
    }
}

impl Decode for f64 {
    fn decode<D: Decoder>(d: &mut D) -> Result<f64, D::Error> {
        d.read_f64()
    }
}

impl<'a, T: Encode> Encode for &'a T {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        <T>::encode(&**self, e)
    }
}

impl<T: Encode> Encode for Box<T> {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        <T>::encode(&**self, e)
    }
}

impl<T: Decode> Decode for Box<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<Box<T>, D::Error> {
        Ok(Box::new(<T>::decode(d)?))
    }
}

impl<T: Encode> Encode for std::rc::Rc<T> {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        <T>::encode(&**self, e)
    }
}

impl<T: Decode> Decode for std::rc::Rc<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<std::rc::Rc<T>, D::Error> {
        Ok(std::rc::Rc::new(<T>::decode(d)?))
    }
}

impl<T: Encode> Encode for std::cell::RefCell<T> {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        <T>::encode(&*self.borrow(), e)
    }
}

impl<T: Decode> Decode for std::cell::RefCell<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<std::cell::RefCell<T>, D::Error> {
        Ok(std::cell::RefCell::new(<T>::decode(d)?))
    }
}

impl<T: Encode + Copy> Encode for std::cell::Cell<T> {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        <T>::encode(&self.get(), e)
    }
}

impl<T: Decode> Decode for std::cell::Cell<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<std::cell::Cell<T>, D::Error> {
        Ok(std::cell::Cell::new(<T>::decode(d)?))
    }
}

impl<T: Encode> Encode for Vec<T> {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_list(self)
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<Vec<T>, D::Error> {
        d.read_list()
    }
}

impl<T: Encode> Encode for [T] {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_list(self)
    }
}

impl<T: Decode> Decode for Box<[T]> {
    fn decode<D: Decoder>(d: &mut D) -> Result<Box<[T]>, D::Error> {
        let vec = <Vec<T>>::decode(d)?;

        Ok(vec.into_boxed_slice())
    }
}

impl<K: Encode, V: Encode, S> Encode for std::collections::HashMap<K, V, S> {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_map(self)
    }
}

impl<K: Decode, V: Decode, S> Decode for std::collections::HashMap<K, V, S>
where
    K: Eq + std::hash::Hash,
    S: Default + std::hash::BuildHasher,
{
    fn decode<D: Decoder>(d: &mut D) -> Result<std::collections::HashMap<K, V, S>, D::Error> {
        d.read_map()
    }
}

impl<K: Encode, V: Encode> Encode for std::collections::BTreeMap<K, V> {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_map(self)
    }
}

impl<K: Decode, V: Decode> Decode for std::collections::BTreeMap<K, V>
where
    K: Ord
{
    fn decode<D: Decoder>(d: &mut D) -> Result<std::collections::BTreeMap<K, V>, D::Error> {
        d.read_map()
    }
}

impl<T: Encode> Encode for Option<T> {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        match self {
            Some(ref t) => {
                e.emit_u8(0)?;
                <T as Encode>::encode(t, e)
            },
            None => e.emit_u8(1),
        }
    }
}

impl<T: Decode> Decode for Option<T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<Option<T>, D::Error> {
        match d.read_u8()? {
            0 => Ok(Some(<T as Decode>::decode(d)?)),
            1 => Ok(None),
            _ => Err(d.error("invalid discriminant")),
        }
    }
}

impl<A: Encode, B: Encode> Encode for Result<A, B> {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        match self {
            Ok(ref a) => {
                e.emit_u8(0)?;
                <A as Encode>::encode(a, e)
            },
            Err(ref b) => {
                e.emit_u8(1)?;
                <B as Encode>::encode(b, e)
            },
        }
    }
}

impl<A: Decode, B: Decode> Decode for Result<A, B> {
    fn decode<D: Decoder>(d: &mut D) -> Result<Result<A, B>, D::Error> {
        match d.read_u8()? {
            0 => Ok(Ok(<A as Decode>::decode(d)?)),
            1 => Ok(Err(<B as Decode>::decode(d)?)),
            _ => Err(d.error("invalid discriminant")),
        }
    }
}

macro_rules! impl_tuples {
    () => {};

    ($t0:ident $(, $t:ident)*) => {
        impl_tuples!($($t),*);

        impl<$t0: Encode $(, $t: Encode)*> Encode for ($t0, $($t),*) {
            #[allow(non_snake_case)]
            fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
                let (ref $t0, $(ref $t),*) = self;

                <$t0>::encode($t0, e)?;
                $(<$t>::encode($t, e)?;)*

                Ok(())
            }
        }

        impl<$t0: Decode $(, $t: Decode)*> Decode for ($t0, $($t),*) {
            #[allow(non_snake_case)]
            fn decode<D: Decoder>(d: &mut D) -> Result<($t0, $($t),*), D::Error> {
                let $t0 = <$t0>::decode(d)?;
                $(let $t = <$t>::decode(d)?;)*

                Ok(($t0, $($t),*))
            }
        }
    };
}

impl_tuples!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
