pub mod deserialize;
pub mod error;
pub mod read;
pub mod serialize;
pub mod write;

pub type Result<T> = core::result::Result<T, error::Error>;

use serde::de;
use serde::ser;

pub fn to_writer<S, W>(output: W, value: &S) -> Result<usize>
where
	S: ser::Serialize,
	W: std::io::Write,
{
	value.serialize(&mut serialize::Serializer::new(write::IoWriter::new(
		output,
	)))
}

pub fn to_vec<S>(value: &S) -> Result<Vec<u8>>
where
	S: ser::Serialize,
{
	let mut vec = vec![];
	to_writer(&mut vec, value)?;
	Ok(vec)
}

pub fn to_slice<S>(slice: &mut [u8], value: &S) -> Result<usize>
where
	S: ser::Serialize,
{
	value.serialize(&mut serialize::Serializer::new(write::SliceWriter::new(slice)))
}

pub fn from_reader<T, R>(reader: R) -> Result<T>
where
	T: de::DeserializeOwned,
	R: std::io::Read,
{
	let mut deserializer =
		deserialize::Deserializer::new(crate::codec::read::IoReader::new(reader, 128));
	let value = de::Deserialize::deserialize(&mut deserializer)?;
	Ok(value)
}

pub fn from_slice<'a, T>(slice: &'a [u8]) -> Result<T>
where
	T: de::Deserialize<'a>,
{
	let mut deserializer =
		deserialize::Deserializer::new(crate::codec::read::SliceReader::new(slice));
	let value = de::Deserialize::deserialize(&mut deserializer)?;
	Ok(value)
}
