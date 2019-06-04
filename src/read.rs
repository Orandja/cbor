use crate::error::*;
use crate::Result;
use byteorder::{BigEndian, ByteOrder};

pub enum EitherLifetime<'c, 'o> {
	Current(&'c [u8]),
	Other(&'o [u8]),
}

const LENGHT_U8: usize = 1;
const LENGHT_U16: usize = 2;
const LENGHT_U32: usize = 4;
const LENGHT_U64: usize = 8;

pub trait Reader<'r> {
	fn read_bytes<'a>(&'a mut self, size: usize) -> Result<EitherLifetime<'a, 'r>>;

	fn read_u8(&mut self) -> Result<u8> {
		Ok(match self.read_bytes(LENGHT_U8)? {
			EitherLifetime::Current(bytes) => bytes[0],
			EitherLifetime::Other(bytes) => bytes[0],
		})
	}

	fn read_u16(&mut self) -> Result<u16> {
		Ok(match self.read_bytes(LENGHT_U16)? {
			EitherLifetime::Current(bytes) => BigEndian::read_u16(bytes),
			EitherLifetime::Other(bytes) => BigEndian::read_u16(bytes),
		})
	}

	fn read_u32(&mut self) -> Result<u32> {
		Ok(match self.read_bytes(LENGHT_U32)? {
			EitherLifetime::Current(bytes) => BigEndian::read_u32(bytes),
			EitherLifetime::Other(bytes) => BigEndian::read_u32(bytes),
		})
	}

	fn read_u64(&mut self) -> Result<u64> {
		Ok(match self.read_bytes(LENGHT_U64)? {
			EitherLifetime::Current(bytes) => BigEndian::read_u64(bytes),
			EitherLifetime::Other(bytes) => BigEndian::read_u64(bytes),
		})
	}

	fn read_f32(&mut self) -> Result<f32> {
		Ok(match self.read_bytes(LENGHT_U32)? {
			EitherLifetime::Current(bytes) => BigEndian::read_f32(bytes),
			EitherLifetime::Other(bytes) => BigEndian::read_f32(bytes),
		})
	}

	fn read_f64(&mut self) -> Result<f64> {
		Ok(match self.read_bytes(LENGHT_U64)? {
			EitherLifetime::Current(bytes) => BigEndian::read_f64(bytes),
			EitherLifetime::Other(bytes) => BigEndian::read_f64(bytes),
		})
	}
}

pub struct SliceReader<'r> {
	slice: &'r [u8],
	index: usize,
}

impl<'r> SliceReader<'r> {
	#[inline]
	fn end(&self, size: usize) -> Result<usize> {
		match self.index.checked_add(size) {
			Some(end) if end <= self.slice.len() => Ok(end),
			_ => Err(Error::Message("Try to read after the end of the slice.")),
		}
	}

	pub fn new(slice: &'r [u8]) -> Self {
		SliceReader {
			slice: slice,
			index: 0,
		}
	}
}

impl<'r> Reader<'r> for SliceReader<'r> {
	#[inline]
	fn read_bytes<'a>(&'a mut self, size: usize) -> Result<EitherLifetime<'a, 'r>> {
		let end = self.end(size)?;
		let bytes = &self.slice[self.index..end];
		self.index = end;
		Ok(EitherLifetime::Other(bytes))
	}
	#[inline]
	fn read_u8(&mut self) -> Result<u8> {
		let end = self.end(LENGHT_U8)?;
		let value = self.slice[self.index];
		self.index = end;
		Ok(value)
	}
	#[inline]
	fn read_u16(&mut self) -> Result<u16> {
		let end = self.end(LENGHT_U16)?;
		let value = BigEndian::read_u16(&self.slice[self.index..end]);
		self.index = end;
		Ok(value)
	}
	#[inline]
	fn read_u32(&mut self) -> Result<u32> {
		let end = self.end(LENGHT_U32)?;
		let value = BigEndian::read_u32(&self.slice[self.index..end]);
		self.index = end;
		Ok(value)
	}
	#[inline]
	fn read_u64(&mut self) -> Result<u64> {
		let end = self.end(LENGHT_U64)?;
		let value = BigEndian::read_u64(&self.slice[self.index..end]);
		self.index = end;
		Ok(value)
	}
	#[inline]
	fn read_f32(&mut self) -> Result<f32> {
		let end = self.end(LENGHT_U32)?;
		let value = BigEndian::read_f32(&self.slice[self.index..end]);
		self.index = end;
		Ok(value)
	}
	#[inline]
	fn read_f64(&mut self) -> Result<f64> {
		let end = self.end(LENGHT_U64)?;
		let value = BigEndian::read_f64(&self.slice[self.index..end]);
		self.index = end;
		Ok(value)
	}
}

use std::io;

pub struct IoReader<R: io::Read> {
	reader: R,
	scratch: Vec<u8>,
	limited: bool,
}

impl<R: io::Read> IoReader<R> {
	pub fn new(io: R) -> Self {
		IoReader {
			reader: io,
			scratch: Vec::with_capacity(9),
			limited: false,
		}
	}

	pub fn with_limit(io: R, capacity: usize) -> Self {
		IoReader {
			reader: io,
			scratch: Vec::with_capacity(capacity),
			limited: true,
		}
	}

	#[inline]
	fn reserve(&mut self, size: usize) -> Result<()> {
		if size > self.scratch.capacity() {
			if self.limited {
				return Err(Error::Message(
					"Buffer limit exeed reach when reading a io::read element",
				));
			} else {
				self.scratch.reserve(size - self.scratch.capacity());
			}
		}
		unsafe { self.scratch.set_len(size) };
		Ok(())
	}
}

impl<'r, R: io::Read> Reader<'r> for IoReader<R> {
	#[inline]
	fn read_bytes<'a>(&'a mut self, size: usize) -> Result<EitherLifetime<'a, 'r>> {
		self.reserve(size)?;
		self.reader.read(&mut self.scratch).unwrap();
		Ok(EitherLifetime::Current(&self.scratch))
	}

	#[inline]
	fn read_u8(&mut self) -> Result<u8> {
		self.reserve(LENGHT_U8)?;
		self.reader.read(&mut self.scratch).unwrap();
		Ok(self.scratch[0])
	}
	#[inline]
	fn read_u16(&mut self) -> Result<u16> {
		self.reserve(LENGHT_U16)?;
		self.reader.read(&mut self.scratch).unwrap();
		Ok(BigEndian::read_u16(&self.scratch))
	}

	#[inline]
	fn read_u32(&mut self) -> Result<u32> {
		self.reserve(LENGHT_U32)?;
		self.reader.read(&mut self.scratch).unwrap();
		Ok(BigEndian::read_u32(&self.scratch))
	}

	#[inline]
	fn read_u64(&mut self) -> Result<u64> {
		self.reserve(LENGHT_U64)?;
		self.reader.read(&mut self.scratch).unwrap();
		Ok(BigEndian::read_u64(&self.scratch))
	}

	#[inline]
	fn read_f32(&mut self) -> Result<f32> {
		self.reserve(LENGHT_U32)?;
		self.reader.read(&mut self.scratch).unwrap();
		Ok(BigEndian::read_f32(&self.scratch))
	}

	#[inline]
	fn read_f64(&mut self) -> Result<f64> {
		self.reserve(LENGHT_U64)?;
		self.reader.read(&mut self.scratch).unwrap();
		Ok(BigEndian::read_f64(&self.scratch))
	}
}
