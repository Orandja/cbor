/// An enum error that represent all possible errors comming from
/// this codec.

#[derive(Debug)]
pub enum Error {
	/// This error is generated when a serde error is encounter
	/// it describe when the error occured (SerdeWhen::{Serialization, Deserialization})
	/// and what the error is in textual form.
	Serde(SerdeWhen, String),

	/// An error which is not related to this codec.
	/// Can be one of :
	/// - std::io::Error
	/// - std::str::Utf8Error
	/// - std::num::TryFromIntError
	Other(OtherKind, Box<dyn std::error::Error>),

	/// An error that is yet to be defined here.
	TemporalError(&'static str),
}

/// Define when a serde error occured
#[derive(Debug)]
pub enum SerdeWhen {
	/// During a serialization
	Serialization,
	/// During a deserialization
	Deserialization,
}

/// Define the other kind of error.
#[derive(Debug)]
pub enum OtherKind {
	/// An io error occured
	Io,
	/// Malformated utf8
	Utf8,
	/// An illegal decoded number like -(u64::max)
	Numerical,
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "")
	}
}

impl std::error::Error for Error {}

impl serde::de::Error for Error {
	fn custom<T: std::fmt::Display>(msg: T) -> Self {
		Error::Serde(SerdeWhen::Deserialization, msg.to_string())
	}
}

impl serde::ser::Error for Error {
	fn custom<T: std::fmt::Display>(msg: T) -> Self {
		Error::Serde(SerdeWhen::Deserialization, msg.to_string())
	}
}

impl std::convert::From<std::io::Error> for Error {
	fn from(item: std::io::Error) -> Self {
		Error::Other(OtherKind::Io, Box::new(item))
	}
}

impl std::convert::From<std::str::Utf8Error> for Error {
	fn from(item: std::str::Utf8Error) -> Self {
		Error::Other(OtherKind::Utf8, Box::new(item))
	}
}

impl std::convert::From<std::num::TryFromIntError> for Error {
	fn from(item: std::num::TryFromIntError) -> Self {
		Error::Other(OtherKind::Numerical, Box::new(item))
	}
}
