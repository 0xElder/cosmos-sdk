use crate::buffer::WriterFactory;
use crate::decoder::{decode, DecodeError, Decoder};
use crate::encoder::{EncodeError, Encoder};
use crate::fields::FieldTypes;
use crate::mem::MemoryManager;
use crate::state_object::field_types::unnamed_struct_type;
use crate::state_object::value_field::ObjectFieldValue;
use crate::structs::{StructDecodeVisitor, StructEncodeVisitor, StructType};
use crate::value::Value;

/// This trait is implemented for types that can be used as tuples of value fields in state objects.
pub trait ObjectValue {
    /// The object value types as field types.
    type FieldTypes<'a>: FieldTypes;
    /// The type that is used when inputting object values to functions.
    type In<'a>;
    /// The type that is used in function return values.
    type Out<'a>;
    /// The associated "pseudo-struct" type for the object value.
    const PSEUDO_TYPE: StructType<'static>;

    /// Encode each part of the value in reverse order.
    fn encode<'a, E: Encoder>(value: &Self::In<'a>, encoder: &mut E) -> Result<(), EncodeError>;

    /// Decode the value from the decoder.
    fn decode<'a, D: Decoder<'a>>(decoder: &mut D, mem: &'a MemoryManager) -> Result<Self::Out<'a>, DecodeError>;
}

impl ObjectValue for () {
    type FieldTypes<'a> = ();
    type In<'a> = ();
    type Out<'a> = ();
    const PSEUDO_TYPE: StructType<'static> = unnamed_struct_type::<Self::FieldTypes<'static>>();

    fn encode<'a, E: Encoder>(value: &Self::In<'a>, encoder: &mut E) -> Result<(), EncodeError> {
        Ok(())
    }

    fn decode<'a, D: Decoder<'a>>(decoder: &mut D, mem: &'a MemoryManager) -> Result<Self::Out<'a>, DecodeError> {
        Ok(())
    }
}

impl<A: ObjectFieldValue> ObjectValue for A {
    type FieldTypes<'a> = (<<A as ObjectFieldValue>::In<'a> as Value<'a>>::Type,);
    type In<'a> = A::In<'a>;
    type Out<'a> = A::Out<'a>;
    const PSEUDO_TYPE: StructType<'static> = unnamed_struct_type::<Self::FieldTypes<'static>>();

    fn encode<'a, E: Encoder>(value: &Self::In<'a>, encoder: &mut E) -> Result<(), EncodeError> {
        todo!()
    }

    fn decode<'a, D: Decoder<'a>>(decoder: &mut D, mem: &'a MemoryManager) -> Result<Self::Out<'a>, DecodeError> {
        struct Visitor<'a, A:Value<'a>>(A::DecodeState);
        unsafe impl <'a, A: Value<'a>> StructDecodeVisitor<'a> for Visitor<'a, A> {
            fn decode_field<D: Decoder<'a>>(&mut self, index: usize, decoder: &mut D) -> Result<(), DecodeError> {
                match index {
                    0 => <A as Value<'a>>::visit_decode_state(&mut self.0, decoder),
                    _ => Err(DecodeError::UnknownFieldNumber),
                }
            }
        }

        let mut visitor: Visitor<'a, A::Out<'a>> = Visitor(Default::default());
        decoder.decode_struct(&mut visitor, &Self::PSEUDO_TYPE)?;
        Ok(<A::Out<'a> as Value<'a>>::finish_decode_state(visitor.0, mem)?)
    }
}

impl<A: ObjectFieldValue> ObjectValue for (A,) {
    type FieldTypes<'a> = (<<A as ObjectFieldValue>::In<'a> as Value<'a>>::Type,);
    type In<'a> = (A::In<'a>,);
    type Out<'a> = (A::Out<'a>,);
    const PSEUDO_TYPE: StructType<'static> = unnamed_struct_type::<Self::FieldTypes<'static>>();

    fn encode<'a, E: Encoder>(value: &Self::In<'a>, encoder: &mut E) -> Result<(), EncodeError> {
        struct Visitor<'b, A>(&'b (A,));
        unsafe impl <'b, 'a:'b, A: Value<'a>> StructEncodeVisitor for Visitor<'b, A> {
            fn encode_field<E: Encoder>(&self, index: usize, encoder: &mut E) -> Result<(), EncodeError> {
                match index {
                    0 => <A as Value<'a>>::encode(&self.0.0, encoder),
                    _ =>
                        Err(EncodeError::UnknownError),
                }
            }
        }

        encoder.encode_struct(&Visitor(value), &Self::PSEUDO_TYPE)
    }

    fn decode<'a, D: Decoder<'a>>(decoder: &mut D, mem: &'a MemoryManager) -> Result<Self::Out<'a>, DecodeError> {
        Ok((<A as ObjectValue>::decode(decoder, mem)?,))
    }
}

impl<A: ObjectFieldValue, B: ObjectFieldValue> ObjectValue for (A, B) {
    type FieldTypes<'a> = (<<A as ObjectFieldValue>::In<'a> as Value<'a>>::Type, <<B as ObjectFieldValue>::In<'a> as Value<'a>>::Type);
    type In<'a> = (A::In<'a>, B::In<'a>);
    type Out<'a> = (A::Out<'a>, B::Out<'a>);
    const PSEUDO_TYPE: StructType<'static> = unnamed_struct_type::<Self::FieldTypes<'static>>();

    fn encode<'a, E: Encoder>(value: &Self::In<'a>, encoder: &mut E) -> Result<(), EncodeError> {
        struct EncodeVisitor<'b, A, B>(&'b (A, B));
        unsafe impl <'b, 'a:'b, A: Value<'a>, B: Value<'a>> StructEncodeVisitor for EncodeVisitor<'b, A, B> {
            fn encode_field<E: Encoder>(&self, index: usize, encoder: &mut E) -> Result<(), EncodeError> {
                match index {
                    0 => <A as Value<'a>>::encode(&self.0.0, encoder),
                    1 => <B as Value<'a>>::encode(&self.0.1, encoder),
                    _ =>
                        Err(EncodeError::UnknownError),
                }
            }
        }

        encoder.encode_struct(&EncodeVisitor(value), &Self::PSEUDO_TYPE)
    }

    fn decode<'a, D: Decoder<'a>>(decoder: &mut D, mem: &'a MemoryManager) -> Result<Self::Out<'a>, DecodeError> {
        struct Visitor<'a, A:Value<'a>, B:Value<'a>>(A::DecodeState, B::DecodeState);
        unsafe impl <'a, A: Value<'a>, B: Value<'a>> StructDecodeVisitor<'a> for Visitor<'a, A, B> {
            fn decode_field<D: Decoder<'a>>(&mut self, index: usize, decoder: &mut D) -> Result<(), DecodeError> {
                match index {
                    0 => <A as Value<'a>>::visit_decode_state(&mut self.0, decoder),
                    1 => <B as Value<'a>>::visit_decode_state(&mut self.1, decoder),
                    _ => Err(DecodeError::UnknownFieldNumber),
                }
            }
        }

        let mut visitor: Visitor<'a, A::Out<'a>, B::Out<'a>> = Visitor(Default::default(), Default::default());
        decoder.decode_struct(&mut visitor, &Self::PSEUDO_TYPE)?;
        Ok((
            <A::Out<'a> as Value<'a>>::finish_decode_state(visitor.0, mem)?,
            <B::Out<'a> as Value<'a>>::finish_decode_state(visitor.1, mem)?,
        ))
    }
}

impl<A: ObjectFieldValue, B: ObjectFieldValue, C: ObjectFieldValue> ObjectValue for (A, B, C) {
    type FieldTypes<'a> = (<<A as ObjectFieldValue>::In<'a> as Value<'a>>::Type, <<B as ObjectFieldValue>::In<'a> as Value<'a>>::Type, <<C as ObjectFieldValue>::In<'a> as Value<'a>>::Type);
    type In<'a> = (A::In<'a>, B::In<'a>, C::In<'a>);
    type Out<'a> = (A::Out<'a>, B::Out<'a>, C::Out<'a>);
    const PSEUDO_TYPE: StructType<'static> = unnamed_struct_type::<Self::FieldTypes<'static>>();

    fn encode<'a, E: Encoder>(value: &Self::In<'a>, encoder: &mut E) -> Result<(), EncodeError> {
        struct EncodeVisitor<'b, A, B, C>(&'b (A, B, C));
        unsafe impl <'b, 'a:'b, A: Value<'a>, B: Value<'a>, C: Value<'a>> StructEncodeVisitor for EncodeVisitor<'b, A, B, C> {
            fn encode_field<E: Encoder>(&self, index: usize, encoder: &mut E) -> Result<(), EncodeError> {
                match index {
                    0 => <A as Value<'a>>::encode(&self.0.0, encoder),
                    1 => <B as Value<'a>>::encode(&self.0.1, encoder),
                    2 => <C as Value<'a>>::encode(&self.0.2, encoder),
                    _ =>
                        Err(EncodeError::UnknownError),
                }
            }
        }

        encoder.encode_struct(&EncodeVisitor(value), &Self::PSEUDO_TYPE)
    }

    fn decode<'a, D: Decoder<'a>>(decoder: &mut D, mem: &'a MemoryManager) -> Result<Self::Out<'a>, DecodeError> {
        struct Visitor<'a, A:Value<'a>, B:Value<'a>, C:Value<'a>>(A::DecodeState, B::DecodeState, C::DecodeState);
        unsafe impl <'a, A: Value<'a>, B: Value<'a>, C: Value<'a>> StructDecodeVisitor<'a> for Visitor<'a, A, B, C> {
            fn decode_field<D: Decoder<'a>>(&mut self, index: usize, decoder: &mut D) -> Result<(), DecodeError> {
                match index {
                    0 => <A as Value<'a>>::visit_decode_state(&mut self.0, decoder),
                    1 => <B as Value<'a>>::visit_decode_state(&mut self.1, decoder),
                    2 => <C as Value<'a>>::visit_decode_state(&mut self.2, decoder),
                    _ => Err(DecodeError::UnknownFieldNumber),
                }
            }
        }

        let mut visitor: Visitor<'a, A::Out<'a>, B::Out<'a>, C::Out<'a>> = Visitor(Default::default(), Default::default(), Default::default());
        decoder.decode_struct(&mut visitor, &Self::PSEUDO_TYPE)?;
        Ok((
            <A::Out<'a> as Value<'a>>::finish_decode_state(visitor.0, mem)?,
            <B::Out<'a> as Value<'a>>::finish_decode_state(visitor.1, mem)?,
            <C::Out<'a> as Value<'a>>::finish_decode_state(visitor.2, mem)?,
        ))
    }
}

impl<A: ObjectFieldValue, B: ObjectFieldValue, C: ObjectFieldValue, D: ObjectFieldValue> ObjectValue for (A, B, C, D) {
    type FieldTypes<'a> = (<<A as ObjectFieldValue>::In<'a> as Value<'a>>::Type, <<B as ObjectFieldValue>::In<'a> as Value<'a>>::Type, <<C as ObjectFieldValue>::In<'a> as Value<'a>>::Type, <<D as ObjectFieldValue>::In<'a> as Value<'a>>::Type);
    type In<'a> = (A::In<'a>, B::In<'a>, C::In<'a>, D::In<'a>);
    type Out<'a> = (A::Out<'a>, B::Out<'a>, C::Out<'a>, D::Out<'a>);
    const PSEUDO_TYPE: StructType<'static> = unnamed_struct_type::<Self::FieldTypes<'static>>();

    fn encode<'a, E: Encoder>(value: &Self::In<'a>, encoder: &mut E) -> Result<(), EncodeError> {
        struct EncodeVisitor<'b, A, B, C, D>(&'b (A, B, C, D));
        unsafe impl <'b, 'a:'b, A: Value<'a>, B: Value<'a>, C: Value<'a>, D: Value<'a>> StructEncodeVisitor for EncodeVisitor<'b, A, B, C, D> {
            fn encode_field<E: Encoder>(&self, index: usize, encoder: &mut E) -> Result<(), EncodeError> {
                match index {
                    0 => <A as Value<'a>>::encode(&self.0.0, encoder),
                    1 => <B as Value<'a>>::encode(&self.0.1, encoder),
                    2 => <C as Value<'a>>::encode(&self.0.2, encoder),
                    3 => <D as Value<'a>>::encode(&self.0.3, encoder),
                    _ =>
                        Err(EncodeError::UnknownError),
                }
            }
        }

        encoder.encode_struct(&EncodeVisitor(value), &Self::PSEUDO_TYPE)
    }

    fn decode<'a, DEC: Decoder<'a>>(decoder: &mut DEC, mem: &'a MemoryManager) -> Result<Self::Out<'a>, DecodeError> {
        struct Visitor<'a, A:Value<'a>, B:Value<'a>, C:Value<'a>, D:Value<'a>>(A::DecodeState, B::DecodeState, C::DecodeState, D::DecodeState);
        unsafe impl <'a, A: Value<'a>, B: Value<'a>, C: Value<'a>, D: Value<'a>> StructDecodeVisitor<'a> for Visitor<'a, A, B, C, D> {
            fn decode_field<DEC: Decoder<'a>>(&mut self, index: usize, decoder: &mut DEC) -> Result<(), DecodeError> {
                match index {
                    0 => <A as Value<'a>>::visit_decode_state(&mut self.0, decoder),
                    1 => <B as Value<'a>>::visit_decode_state(&mut self.1, decoder),
                    2 => <C as Value<'a>>::visit_decode_state(&mut self.2, decoder),
                    3 => <D as Value<'a>>::visit_decode_state(&mut self.3, decoder),
                    _ => Err(DecodeError::UnknownFieldNumber),
                }
            }
        }

        let mut visitor: Visitor<'a, A::Out<'a>, B::Out<'a>, C::Out<'a>, D::Out<'a>> = Visitor(Default::default(), Default::default(), Default::default(), Default::default());
        decoder.decode_struct(&mut visitor, &Self::PSEUDO_TYPE)?;
        Ok((
            <A::Out<'a> as Value<'a>>::finish_decode_state(visitor.0, mem)?,
            <B::Out<'a> as Value<'a>>::finish_decode_state(visitor.1, mem)?,
            <C::Out<'a> as Value<'a>>::finish_decode_state(visitor.2, mem)?,
            <D::Out<'a> as Value<'a>>::finish_decode_state(visitor.3, mem)?,
        ))
    }
}

