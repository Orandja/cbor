use crate::cbor::*;
use crate::error::*;
use crate::write::*;
use crate::Result;
use byteorder::{BigEndian, ByteOrder};
use serde::ser;
use std::convert::TryFrom;

type Ok = usize;

pub struct Serializer<W: Writer> {
	writer: W,
	buffer: [u8; 9],
	len_buffer: usize,
}

impl<W: Writer> Serializer<W> {
	pub fn new(writer: W) -> Self {
		Serializer {
			writer: writer,
			buffer: [0u8; 9],
			len_buffer: 0,
		}
	}

	#[inline]
	fn write_header_u8(&mut self, major: u8, header_value: u8) -> Result<Ok> {
		if header_value < SIZE_U8 {
			self.buffer[0] = major << 5 | header_value;
			self.writer.write(&self.buffer[..1])
		} else {
			self.buffer[0] = major << 5 | SIZE_U8;
			self.buffer[1] = header_value;
			self.writer.write(&self.buffer[..2])
		}
	}

	#[inline]
	fn write_header_u16(&mut self, major: u8, header_value: u16) -> Result<Ok> {
		if header_value <= core::u8::MAX as u16 {
			self.write_header_u8(major, header_value as u8)
		} else {
			self.buffer[0] = major << 5 | SIZE_U16;
			BigEndian::write_u16(&mut self.buffer[1..], header_value);
			self.writer.write(&self.buffer[..3])
		}
	}

	#[inline]
	fn write_header_u32(&mut self, major: u8, header_value: u32) -> Result<Ok> {
		if header_value <= core::u16::MAX as u32 {
			self.write_header_u16(major, header_value as u16)
		} else {
			self.buffer[0] = major << 5 | SIZE_U32;
			BigEndian::write_u32(&mut self.buffer[1..], header_value);
			self.writer.write(&self.buffer[..5])
		}
	}

	#[inline]
	fn write_header_u64(&mut self, major: u8, header_value: u64) -> Result<Ok> {
		if header_value <= core::u32::MAX as u64 {
			self.write_header_u32(major, header_value as u32)
		} else {
			self.buffer[0] = major << 5 | SIZE_U64;
			BigEndian::write_u64(&mut self.buffer[1..], header_value);
			self.writer.write(&self.buffer)
		}
	}
}

impl<'a, W: Writer> ser::Serializer for &'a mut Serializer<W> {
	type Ok = Ok;
	type Error = Error;

	type SerializeSeq = SerializeSeq<'a, W>;
	type SerializeTuple = SerializeTuple<'a, W>;
	type SerializeTupleStruct = SerializeTupleStruct<'a, W>;
	type SerializeTupleVariant = SerializeTupleVariant<'a, W>;
	type SerializeMap = SerializeMap<'a, W>;
	type SerializeStruct = SerializeStruct<'a, W>;
	type SerializeStructVariant = SerializeStructVariant<'a, W>;

	#[inline]
	fn serialize_bool(self, value: bool) -> Result<Self::Ok> {
		self.buffer[0] = if value { HEADER_TRUE } else { HEADER_FALSE };
		self.writer.write(&self.buffer[..1])
	}

	#[inline]
	fn serialize_i8(self, value: i8) -> Result<Self::Ok> {
		if value.is_negative() {
			self.write_header_u8(MAJOR_NEGATIVE, u8::try_from(-(value + 1))?)
		} else {
			self.write_header_u8(MAJOR_POSITIVE, value as u8)
		}
	}

	#[inline]
	fn serialize_i16(self, value: i16) -> Result<Self::Ok> {
		if value.is_negative() {
			self.write_header_u16(MAJOR_NEGATIVE, u16::try_from(-(value + 1))?)
		} else {
			self.write_header_u16(MAJOR_POSITIVE, value as u16)
		}
	}

	#[inline]
	fn serialize_i32(self, value: i32) -> Result<Self::Ok> {
		if value.is_negative() {
			self.write_header_u32(MAJOR_NEGATIVE, u32::try_from(-(value + 1))?)
		} else {
			self.write_header_u32(MAJOR_POSITIVE, value as u32)
		}
	}

	#[inline]
	fn serialize_i64(self, value: i64) -> Result<Self::Ok> {
		if value.is_negative() {
			self.write_header_u64(MAJOR_NEGATIVE, u64::try_from(-(value + 1))?)
		} else {
			self.write_header_u64(MAJOR_POSITIVE, value as u64)
		}
	}

	#[inline]
	fn serialize_u8(self, value: u8) -> Result<Self::Ok> {
		self.write_header_u8(MAJOR_POSITIVE, value)
	}

	#[inline]
	fn serialize_u16(self, value: u16) -> Result<Self::Ok> {
		self.write_header_u16(MAJOR_POSITIVE, value)
	}

	#[inline]
	fn serialize_u32(self, value: u32) -> Result<Self::Ok> {
		self.write_header_u32(MAJOR_POSITIVE, value)
	}

	#[inline]
	fn serialize_u64(self, value: u64) -> Result<Self::Ok> {
		self.write_header_u64(MAJOR_POSITIVE, value)
	}

	#[inline]
	fn serialize_f32(self, value: f32) -> Result<Self::Ok> {
		self.buffer[0] = HEADER_FLOAT_U32;
		BigEndian::write_f32(&mut self.buffer[1..], value);
		self.writer.write(&self.buffer[..5])
	}

	#[inline]
	fn serialize_f64(self, value: f64) -> Result<Self::Ok> {
		self.buffer[0] = HEADER_FLOAT_U64;
		BigEndian::write_f64(&mut self.buffer[1..], value);
		self.writer.write(&self.buffer)
	}

	#[inline]
	fn serialize_char(self, value: char) -> Result<Self::Ok> {
		value.encode_utf8(&mut self.buffer[1..5]);
		self.len_buffer = value.len_utf8();
		self.buffer[0] = HEADER_TEXT_START | self.len_buffer as u8;
		self.writer.write(&self.buffer[..=self.len_buffer])
	}

	#[inline]
	fn serialize_str(self, value: &str) -> Result<Self::Ok> {
		self.len_buffer = self.write_header_u64(MAJOR_TEXT, value.len() as u64)?;
		self.len_buffer += self.writer.write(value.as_bytes())?;
		Ok(self.len_buffer)
	}

	#[inline]
	fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok> {
		self.len_buffer = self.write_header_u64(MAJOR_BYTE, value.len() as u64)?;
		self.len_buffer += self.writer.write(value)?;
		Ok(self.len_buffer)
	}

	#[inline]
	fn serialize_none(self) -> Result<Self::Ok> {
		self.buffer[0] = HEADER_NULL;
		self.writer.write(&self.buffer[..1])
	}

	#[inline]
	fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok>
	where
		T: ser::Serialize,
	{
		value.serialize(self)
	}

	#[inline]
	fn serialize_unit(self) -> Result<Self::Ok> {
		self.buffer[0] = HEADER_UNDEFINED;
		self.writer.write(&self.buffer[..1])
	}

	#[inline]
	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
		self.serialize_unit()
	}

	#[inline]
	fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
	where
		T: ser::Serialize,
	{
		value.serialize(self)
	}

	#[inline]
	fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
		match len {
			Some(len) => {
				let lenght = self.write_header_u64(MAJOR_ARRAY, len as u64)?;
				Ok(SerializeSeq {
					se: self,
					size: lenght,
				})
			}
			None => unimplemented!(),
		}
	}

	#[inline]
	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
		let lenght = self.write_header_u64(MAJOR_ARRAY, len as u64)?;
		Ok(SerializeTuple {
			se: self,
			size: lenght,
		})
	}

	#[inline]
	fn serialize_tuple_struct(
		self,
		_name: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleStruct> {
		let lenght = self.write_header_u64(MAJOR_ARRAY, len as u64)?;
		Ok(SerializeTupleStruct {
			se: self,
			size: lenght,
		})
	}

	#[inline]
	fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
		match len {
			Some(len) => {
				let len = self.write_header_u64(MAJOR_MAP, len as u64)?;
				Ok(SerializeMap {
					se: self,
					size: len,
				})
			}
			None => unimplemented!(),
		}
	}

	#[inline]
	fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
		self.len_buffer = self.write_header_u64(MAJOR_MAP, len as u64)?;
		Ok(SerializeStruct {
			se: self,
			size: len,
		})
	}

	#[inline]
	fn serialize_unit_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
	) -> Result<Self::Ok> {
		self.serialize_str(variant)
	}

	#[inline]
	fn serialize_newtype_variant<T: ?Sized>(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		value: &T,
	) -> Result<Self::Ok>
	where
		T: ser::Serialize,
	{
		self.buffer[0] = HEADER_MAP_START | 1;
		let mut lenght = self.writer.write(&self.buffer[..1])?;
		lenght += self.serialize_str(variant)?;
		lenght += value.serialize(self)?;
		Ok(lenght)
	}

	#[inline]
	fn serialize_tuple_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleVariant> {
		self.buffer[0] = HEADER_MAP_START | 1;
		let mut lenght = self.writer.write(&self.buffer[..1])?;
		lenght += self.serialize_str(variant)?;
		lenght += self.write_header_u64(MAJOR_ARRAY, len as u64)?;

		Ok(SerializeTupleVariant {
			se: self,
			size: lenght,
		})
	}

	#[inline]
	fn serialize_struct_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeStructVariant> {
		self.buffer[0] = HEADER_MAP_START | 1;
		let mut lenght = self.writer.write(&self.buffer[..1])?;
		lenght += self.serialize_str(variant)?;
		lenght += self.write_header_u64(MAJOR_MAP, len as u64)?;
		Ok(SerializeStructVariant {
			se: self,
			size: lenght,
		})
	}

	#[inline]
	fn is_human_readable(&self) -> bool {
		false
	}
}

pub struct SerializeSeq<'a, W: Writer> {
	se: &'a mut Serializer<W>,
	size: usize,
}

impl<'a, W: Writer> ser::SerializeSeq for SerializeSeq<'a, W> {
	type Ok = Ok;
	type Error = Error;

	#[inline]
	fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
	where
		T: ser::Serialize,
	{
		Ok({
			self.size += value.serialize(&mut *self.se)?;
		})
	}

	#[inline]
	fn end(self) -> Result<Self::Ok> {
		Ok(self.size)
	}
}

pub struct SerializeTuple<'a, W: Writer> {
	se: &'a mut Serializer<W>,
	size: usize,
}

impl<'a, W: Writer> ser::SerializeTuple for SerializeTuple<'a, W> {
	type Ok = Ok;
	type Error = Error;

	#[inline]
	fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
	where
		T: ser::Serialize,
	{
		self.size += value.serialize(&mut *self.se)?;
		Ok(())
	}

	#[inline]
	fn end(self) -> Result<Self::Ok> {
		Ok(self.size)
	}
}

pub struct SerializeTupleStruct<'a, W: Writer> {
	se: &'a mut Serializer<W>,
	size: usize,
}

impl<'a, W: Writer> ser::SerializeTupleStruct for SerializeTupleStruct<'a, W> {
	type Ok = Ok;
	type Error = Error;

	#[inline]
	fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
	where
		T: ser::Serialize,
	{
		self.size += value.serialize(&mut *self.se)?;
		Ok(())
	}

	#[inline]
	fn end(self) -> Result<Self::Ok> {
		Ok(self.size)
	}
}

pub struct SerializeTupleVariant<'a, W: Writer> {
	se: &'a mut Serializer<W>,
	size: usize,
}

impl<'a, W: Writer> ser::SerializeTupleVariant for SerializeTupleVariant<'a, W> {
	type Ok = Ok;
	type Error = Error;

	#[inline]
	fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<()>
	where
		T: ser::Serialize,
	{
		self.size += value.serialize(&mut *self.se)?;
		Ok(())
	}

	#[inline]
	fn end(self) -> Result<Self::Ok> {
		Ok(self.size)
	}
}

pub struct SerializeMap<'a, W: Writer> {
	se: &'a mut Serializer<W>,
	size: usize,
}

impl<'a, W: Writer> ser::SerializeMap for SerializeMap<'a, W> {
	type Ok = Ok;
	type Error = Error;

	#[inline]
	fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<()>
	where
		T: ser::Serialize,
	{
		self.size += key.serialize(&mut *self.se)?;
		Ok(())
	}

	#[inline]
	fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<()>
	where
		T: ser::Serialize,
	{
		self.size += value.serialize(&mut *self.se)?;
		Ok(())
	}

	#[inline]
	fn end(self) -> Result<Self::Ok> {
		Ok(self.size)
	}
}

pub struct SerializeStruct<'a, W: Writer> {
	se: &'a mut Serializer<W>,
	size: usize,
}

impl<'a, W: Writer> ser::SerializeStruct for SerializeStruct<'a, W> {
	type Ok = Ok;
	type Error = Error;

	#[inline]
	fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
	where
		T: ser::Serialize,
	{
		self.size += ser::Serializer::serialize_str(&mut *self.se, key)?;
		self.size += value.serialize(&mut *self.se)?;
		Ok(())
	}

	#[inline]
	fn end(self) -> Result<Self::Ok> {
		Ok(self.size)
	}
}

pub struct SerializeStructVariant<'a, W: Writer> {
	se: &'a mut Serializer<W>,
	size: usize,
}

impl<'a, W: Writer> ser::SerializeStructVariant for SerializeStructVariant<'a, W> {
	type Ok = Ok;
	type Error = Error;

	#[inline]
	fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
	where
		T: ser::Serialize,
	{
		self.size += ser::Serializer::serialize_str(&mut *self.se, key)?;
		self.size += value.serialize(&mut *self.se)?;
		Ok(())
	}

	#[inline]
	fn end(self) -> Result<Self::Ok> {
		Ok(self.size)
	}
}
