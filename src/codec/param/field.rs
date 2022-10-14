use bytes::{Buf, Bytes};
use paramdex_rs::ParamdefEndian;

struct FieldDeserializer {
    buffer: Bytes,
    offset: u8,
    endian: ParamdefEndian,
}

impl FieldDeserializer<> {

    pub fn new(buffer: Bytes, endian: ParamdefEndian) -> FieldDeserializer {
        FieldDeserializer { buffer, offset: 0, endian }
    }

    pub fn deserialize_field<T: FieldDeserializable>(&mut self) -> T {
        let out = <T>::deserialize(&mut self.buffer, self.endian);
        if let Some(bits) = <T>::BITS {
            self.offset += bits;
            self.offset = bits % 8;
        }
        out
    }

}

trait FieldDeserializable: Sized + Copy {
    const BITS: Option<u8>;
    fn deserialize(bytes: &mut Bytes, endian: ParamdefEndian) -> Self;
}
macro_rules! simple_field {
    ($typ: ty, $be:path, $le:path) => {
        impl FieldDeserializable for $typ {
            const BITS: Option<u8> = None;
            fn deserialize(bytes: &mut Bytes, endian: ParamdefEndian) -> Self {
                match endian {
                    ParamdefEndian::Little => $le(bytes),
                    ParamdefEndian::Big => $be(bytes),
                }
            }
        }
    };
}

simple_field!(i16, Bytes::get_i16, Bytes::get_i16_le);
simple_field!(u16, Bytes::get_u16, Bytes::get_u16_le);
simple_field!(i32, Bytes::get_i32, Bytes::get_i32_le);
simple_field!(u32, Bytes::get_u32, Bytes::get_u32_le);
simple_field!(f32, Bytes::get_f32, Bytes::get_f32_le);
simple_field!(f64, Bytes::get_f64, Bytes::get_f64_le);

impl FieldDeserializable for u8 {
    const BITS: Option<u8> = None;
    fn deserialize(bytes: &mut Bytes, _endian: ParamdefEndian) -> Self {
        bytes.get_u8()
    }
}

impl FieldDeserializable for i8 {
    const BITS: Option<u8> = None;
    fn deserialize(bytes: &mut Bytes, _endian: ParamdefEndian) -> Self {
        bytes.get_i8()
    }
}

mod complex_types {
    use bytes::{Buf, Bytes};
    use paramdex_rs::ParamdefEndian;

    use crate::codec::param::field::FieldDeserializable;

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash)]
    pub struct Bits<const BITS: u8> {
        inner: u32,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash)]
    pub struct Bool32(pub bool);

    impl FieldDeserializable for Bool32 {
        const BITS: Option<u8> = None;
        fn deserialize(bytes: &mut Bytes, _endian: ParamdefEndian) -> Self {
            match bytes.get_u32() {
                0 => Bool32(false),
                _ => Bool32(true),
            }
        }
    }
}



