use crate::cbor::*;
use crate::error::*;
use crate::read::*;
use crate::Result;
use serde::de;
use std::convert::TryFrom;

pub struct Deserializer<R> {
	reader: R,
	peek: Option<u8>,
}

impl<'de, R: Reader<'de>> Deserializer<R> {
	pub fn new(reader: R) -> Self {
		Deserializer {
			reader: reader,
			peek: None,
		}
	}

	#[inline]
	fn peek(&mut self) -> Result<u8> {
		match self.peek {
			Some(val) => Ok(val),
			None => {
				self.peek = Some(self.reader.read_u8()?);
				Ok(self.peek.unwrap())
			}
		}
	}

	#[inline]
	fn consume(&mut self) {
		self.peek = None;
	}

	#[inline]
	fn peek_and_consume(&mut self) -> Result<u8> {
		let peek = self.peek();
		self.consume();
		peek
	}

	#[inline]
	fn deserialize_f16<V>(&mut self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		if self.peek_and_consume()? == HEADER_FLOAT_U16 {
			visitor.visit_f32(half::f16::from_bits(self.reader.read_u16()?).into())
		} else {
			Err(Error::TemporalError("Not a floating point"))
		}
	}
}

impl<'de, 'r, R> serde::Deserializer<'de> for &'r mut Deserializer<R>
where
	R: Reader<'de>,
{
	type Error = Error;

	#[inline]
	fn deserialize_any<V>(self, _: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		unimplemented!()
	}

	#[inline]
	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		let peek = self.peek_and_consume()?;
		if peek == HEADER_POSITIVE_U8 {
			visitor.visit_u8(self.reader.read_u8()?)
		} else if HEADER_POSITIVE_START <= peek && peek < HEADER_POSITIVE_U8 {
			visitor.visit_u8(peek & 0x1F)
		} else {
			Err(Error::TemporalError("Not an unsigned int"))
		}
	}

	#[inline]
	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		if self.peek()? == HEADER_POSITIVE_U16 {
			self.consume();
			visitor.visit_u16(self.reader.read_u16()?)
		} else {
			self.deserialize_u8(visitor)
		}
	}

	#[inline]
	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		if self.peek()? == HEADER_POSITIVE_U32 {
			self.consume();
			visitor.visit_u32(self.reader.read_u32()?)
		} else {
			self.deserialize_u16(visitor)
		}
	}

	#[inline]
	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		if self.peek()? == HEADER_POSITIVE_U64 {
			self.consume();
			visitor.visit_u64(self.reader.read_u64()?)
		} else {
			self.deserialize_u32(visitor)
		}
	}

	#[inline]
	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		let peek = self.peek_and_consume()?;
		if peek == HEADER_POSITIVE_U8 {
			visitor.visit_u8(self.reader.read_u8()?)
		} else if peek == HEADER_NEGATIVE_U8 {
			visitor.visit_i8(-1 - i8::try_from(self.reader.read_u8()?)?)
		} else if HEADER_POSITIVE_START <= peek && peek < HEADER_POSITIVE_U8 {
			visitor.visit_u8(peek & 0x1F)
		} else if HEADER_NEGATIVE_START <= peek && peek < HEADER_NEGATIVE_U8 {
			visitor.visit_i8(-1 - i8::try_from(peek & 0x1F)?)
		} else {
			Err(Error::TemporalError("Not a signed int"))
		}
	}

	#[inline]
	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		let peek = self.peek()?;
		if peek == HEADER_POSITIVE_U16 {
			self.consume();
			visitor.visit_u16(self.reader.read_u16()?)
		} else if peek == HEADER_NEGATIVE_U16 {
			self.consume();
			visitor.visit_i16(-1 - (i16::try_from(self.reader.read_u16()?)?))
		} else {
			self.deserialize_i8(visitor)
		}
	}

	#[inline]
	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		let peek = self.peek()?;
		if peek == HEADER_POSITIVE_U32 {
			self.consume();
			visitor.visit_u32(self.reader.read_u32()?)
		} else if peek == HEADER_NEGATIVE_U32 {
			self.consume();
			visitor.visit_i32(-1 - (i32::try_from(self.reader.read_u32()?)?))
		} else {
			self.deserialize_i16(visitor)
		}
	}

	#[inline]
	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		let peek = self.peek()?;
		if peek == HEADER_POSITIVE_U64 {
			self.consume();
			visitor.visit_u64(self.reader.read_u64()?)
		} else if peek == HEADER_NEGATIVE_U64 {
			self.consume();
			visitor.visit_i64(-1 - (i64::try_from(self.reader.read_u64()?)?))
		} else {
			self.deserialize_i32(visitor)
		}
	}

	#[inline]
	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		let peek = self.peek_and_consume()?;
		if peek == HEADER_TRUE {
			visitor.visit_bool(true)
		} else if peek == HEADER_FALSE {
			visitor.visit_bool(false)
		} else {
			Err(Error::TemporalError("Not a boolean"))
		}
	}

	#[inline]
	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		if self.peek()? == HEADER_NULL {
			self.consume();
			visitor.visit_none()
		} else {
			visitor.visit_some(self)
		}
	}

	#[inline]
	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		let size: usize = match self.peek_and_consume()? {
			n if HEADER_TEXT_START <= n && n < HEADER_TEXT_U8 => (n & 0x1F) as usize,
			HEADER_TEXT_U8 => (self.reader.read_u8()?) as usize,
			HEADER_TEXT_U16 => (self.reader.read_u16()?) as usize,
			HEADER_TEXT_U32 => usize::try_from(self.reader.read_u32()?)?,
			HEADER_TEXT_U64 => usize::try_from(self.reader.read_u64()?)?,
			_ => return Err(Error::TemporalError("Not a string")),
		};
		match self.reader.read_bytes(size)? {
			EitherLifetime::Current(bytes) => visitor.visit_str(std::str::from_utf8(bytes)?),
			EitherLifetime::Other(bytes) => visitor.visit_borrowed_str(std::str::from_utf8(bytes)?),
		}
	}

	#[inline]
	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_str(visitor)
	}

	#[inline]
	fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_str(visitor)
	}

	#[inline]
	fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		let size: usize = match self.peek_and_consume()? {
			n if HEADER_BYTE_START <= n && n < HEADER_BYTE_U8 => (n & 0x1F) as usize,
			HEADER_BYTE_U8 => (self.reader.read_u8()?) as usize,
			HEADER_BYTE_U16 => (self.reader.read_u16()?) as usize,
			HEADER_BYTE_U32 => usize::try_from(self.reader.read_u32()?)?,
			HEADER_BYTE_U64 => usize::try_from(self.reader.read_u64()?)?,
			_ => return Err(Error::TemporalError("Not a byte")),
		};
		match self.reader.read_bytes(size)? {
			EitherLifetime::Current(bytes) => visitor.visit_bytes(bytes),
			EitherLifetime::Other(bytes) => visitor.visit_borrowed_bytes(bytes),
		}
	}

	#[inline]
	fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_bytes(visitor)
	}

	#[inline]
	fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		if self.peek()? == HEADER_FLOAT_U32 {
			self.consume();
			visitor.visit_f32(self.reader.read_f32()?)
		} else {
			self.deserialize_f16(visitor)
		}
	}

	#[inline]
	fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		if self.peek()? == HEADER_FLOAT_U64 {
			self.consume();
			visitor.visit_f64(self.reader.read_f64()?)
		} else {
			self.deserialize_f32(visitor)
		}
	}

	#[inline]
	fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		if self.peek_and_consume()? == HEADER_UNDEFINED {
			visitor.visit_unit()
		} else {
			Err(Error::TemporalError("Not a unit"))
		}
	}

	#[inline]
	fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		if self.peek_and_consume()? == HEADER_UNDEFINED {
			visitor.visit_unit()
		} else {
			Err(Error::TemporalError("Not a unit struct"))
		}
	}

	#[inline]
	fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_newtype_struct(self)
	}

	#[inline]
	fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		let size: usize = match self.peek_and_consume()? {
			n if HEADER_ARRAY_START <= n && n < HEADER_ARRAY_U8 => (n & 0x1F) as usize,
			HEADER_ARRAY_U8 => (self.reader.read_u8()?) as usize,
			HEADER_ARRAY_U16 => (self.reader.read_u16()?) as usize,
			HEADER_ARRAY_U32 => usize::try_from(self.reader.read_u32()?)?,
			HEADER_ARRAY_U64 => usize::try_from(self.reader.read_u64()?)?,
			_ => return Err(Error::TemporalError("Not a sequence")),
		};
		visitor.visit_seq(SeqAccess {
			de: self,
			len: size,
		})
	}
	#[inline]
	fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_seq(visitor)
	}

	#[inline]
	fn deserialize_tuple_struct<V>(
		self,
		_name: &'static str,
		_len: usize,
		visitor: V,
	) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_seq(visitor)
	}

	#[inline]
	fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		let size: usize = match self.peek_and_consume()? {
			n if HEADER_MAP_START <= n && n < HEADER_MAP_U8 => (n & 0x1F) as usize,
			HEADER_MAP_U8 => (self.reader.read_u8()?) as usize,
			HEADER_MAP_U16 => (self.reader.read_u16()?) as usize,
			HEADER_MAP_U32 => usize::try_from(self.reader.read_u32()?)?,
			HEADER_MAP_U64 => usize::try_from(self.reader.read_u64()?)?,
			_ => return Err(Error::TemporalError("Not a Map")),
		};
		visitor.visit_map(MapAccess {
			de: self,
			len: size,
		})
	}

	#[inline]
	fn deserialize_struct<V>(
		self,
		_name: &'static str,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_map(visitor)
	}

	#[inline]
	fn deserialize_enum<V>(
		self,
		_name: &'static str,
		_variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		match self.peek()? {
			n if HEADER_TEXT_START <= n && n <= HEADER_TEXT_U64 => {
				visitor.visit_enum(VariantAccess { de: self })
			}
			n if n == (HEADER_MAP_START | 1) => {
				self.consume();
				visitor.visit_enum(VariantAccess { de: self })
			}
			_ => Err(Error::TemporalError("Not an enum")),
		}
	}

	#[inline]
	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_str(visitor)
	}

	#[inline]
	fn is_human_readable(&self) -> bool {
		false
	}

	serde::forward_to_deserialize_any! {
		/* bool i8 i16 i32 i64 */ i128 /* u8 u16 u32 u64 */ u128 /* f32 f64 */
		/* unit unit_struct seq tuple tuple_struct  struct map identifier */ ignored_any
		/* char str string bytes byte_buf enum newtype_struct option */
	}
}

struct SeqAccess<'r, R: 'r> {
	de: &'r mut Deserializer<R>,
	len: usize,
}

impl<'de, 'a, R> de::SeqAccess<'de> for SeqAccess<'a, R>
where
	R: Reader<'de>,
{
	type Error = Error;

	#[inline]
	fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
	where
		T: de::DeserializeSeed<'de>,
	{
		if self.len == 0 {
			return Ok(None);
		}
		self.len -= 1;
		Ok(Some(seed.deserialize(&mut *self.de)?))
	}

	#[inline]
	fn size_hint(&self) -> Option<usize> {
		Some(self.len)
	}
}

struct MapAccess<'r, R: 'r> {
	de: &'r mut Deserializer<R>,
	len: usize,
}

impl<'de, 'a, R> de::MapAccess<'de> for MapAccess<'a, R>
where
	R: Reader<'de>,
{
	type Error = Error;

	#[inline]
	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
	where
		K: de::DeserializeSeed<'de>,
	{
		if self.len == 0 {
			return Ok(None);
		}
		self.len -= 1;
		Ok(Some(seed.deserialize(&mut *self.de)?))
	}

	#[inline]
	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
	where
		V: de::DeserializeSeed<'de>,
	{
		Ok(seed.deserialize(&mut *self.de)?)
	}

	#[inline]
	fn size_hint(&self) -> Option<usize> {
		Some(self.len)
	}
}

struct VariantAccess<'a, R> {
	de: &'a mut Deserializer<R>,
}

impl<'de, 'a, R> de::EnumAccess<'de> for VariantAccess<'a, R>
where
	R: Reader<'de>,
{
	type Error = Error;
	type Variant = Self;

	#[inline]
	fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
	where
		V: de::DeserializeSeed<'de>,
	{
		let variant = seed.deserialize(&mut *self.de)?;
		Ok((variant, self))
	}
}

impl<'de, 'a, R> de::VariantAccess<'de> for VariantAccess<'a, R>
where
	R: Reader<'de>,
{
	type Error = Error;

	#[inline]
	fn unit_variant(self) -> Result<()> {
		Ok(())
	}

	#[inline]
	fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
	where
		T: de::DeserializeSeed<'de>,
	{
		seed.deserialize(&mut *self.de)
	}

	#[inline]
	fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		de::Deserializer::deserialize_seq(&mut *self.de, visitor)
	}

	#[inline]
	fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
	where
		V: de::Visitor<'de>,
	{
		de::Deserializer::deserialize_map(&mut *self.de, visitor)
	}
}
