use crate::error::*;
use crate::Result;
use std::io;

pub trait Writer {
	fn write(&mut self, bytes: &[u8]) -> Result<usize>;
}

pub struct SliceWriter<'w> {
	slice: &'w mut [u8],
	index: usize,
}

impl<'w> SliceWriter<'w> {
	pub fn new(slice: &'w mut [u8]) -> Self {
		SliceWriter {
			slice: slice,
			index: 0,
		}
	}

	#[inline]
	fn end(&self, size: usize) -> Result<usize> {
		match self.index.checked_add(size) {
			Some(end) => {
				if end <= self.slice.len() {
					Ok(end)
				} else {
					Err(Error::Message("Try to write after the end of the slice."))
				}
			}
			None => Err(Error::Message("Write into an index that exeed usize::max")),
		}
	}
}

impl<'w> Writer for SliceWriter<'w> {
	#[inline]
	fn write(&mut self, bytes: &[u8]) -> Result<usize> {
		let end = self.end(bytes.len())?;
		unsafe {
			core::ptr::copy_nonoverlapping(
				bytes.as_ptr(),
				self.slice[self.index..end].as_mut_ptr(),
				bytes.len(),
			)
		};
		self.index = end;
		Ok(bytes.len())
	}
}

pub struct IoWriter<W: io::Write> {
	writer: W,
}

impl<W: io::Write> IoWriter<W> {
	pub fn new(io: W) -> Self {
		IoWriter { writer: io }
	}
}

impl<W: io::Write> Writer for IoWriter<W> {
	#[inline]
	fn write(&mut self, bytes: &[u8]) -> Result<usize> {
		Ok(self.writer.write(bytes)?)
	}
}
