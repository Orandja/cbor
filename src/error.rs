#[derive(Debug)]
pub enum Error {
	Message(String),
	Other(Box<dyn std::error::Error>),
	TemporalError(&'static str),
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "")
	}
}

impl std::error::Error for Error {}

impl serde::de::Error for Error {
	fn custom<T: std::fmt::Display>(msg: T) -> Self {
		Error::Message(msg.to_string())
	}
}

impl serde::ser::Error for Error {
	fn custom<T: std::fmt::Display>(msg: T) -> Self {
		Error::Message(msg.to_string())
	}
}

impl std::convert::From<std::io::Error> for Error {
	fn from(item: std::io::Error) -> Self {
		Error::Other(Box::new(item))
	}
}

impl std::convert::From<std::str::Utf8Error> for Error {
	fn from(item: std::str::Utf8Error) -> Self {
		Error::Other(Box::new(item))
	}
}

impl std::convert::From<std::num::TryFromIntError> for Error {
	fn from(item: std::num::TryFromIntError) -> Self {
		Error::Other(Box::new(item))
	}
}
