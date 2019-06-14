// Constants by encoders & decoders.
// used for better understanding in code.

pub const MAJOR_POSITIVE: u8 = 0;
pub const MAJOR_NEGATIVE: u8 = 1;
pub const MAJOR_BYTE: u8 = 2;
pub const MAJOR_TEXT: u8 = 3;
pub const MAJOR_ARRAY: u8 = 4;
pub const MAJOR_MAP: u8 = 5;
pub const MAJOR_TAG: u8 = 6;
pub const MAJOR_PRIMITIVE: u8 = 7;

pub const SIZE_8: u8 = 24;
pub const SIZE_16: u8 = 25;
pub const SIZE_32: u8 = 26;
pub const SIZE_64: u8 = 27;
pub const SIZE_INFINITE: u8 = 31;

pub const HEADER_POSITIVE_START: u8 = MAJOR_POSITIVE << 5;
pub const HEADER_POSITIVE_8: u8 = MAJOR_POSITIVE << 5 | SIZE_8;
pub const HEADER_POSITIVE_16: u8 = MAJOR_POSITIVE << 5 | SIZE_16;
pub const HEADER_POSITIVE_32: u8 = MAJOR_POSITIVE << 5 | SIZE_32;
pub const HEADER_POSITIVE_64: u8 = MAJOR_POSITIVE << 5 | SIZE_64;

pub const HEADER_NEGATIVE_START: u8 = MAJOR_NEGATIVE << 5;
pub const HEADER_NEGATIVE_8: u8 = MAJOR_NEGATIVE << 5 | SIZE_8;
pub const HEADER_NEGATIVE_16: u8 = MAJOR_NEGATIVE << 5 | SIZE_16;
pub const HEADER_NEGATIVE_32: u8 = MAJOR_NEGATIVE << 5 | SIZE_32;
pub const HEADER_NEGATIVE_64: u8 = MAJOR_NEGATIVE << 5 | SIZE_64;

pub const HEADER_BYTE_START: u8 = MAJOR_BYTE << 5;
pub const HEADER_BYTE_8: u8 = MAJOR_BYTE << 5 | SIZE_8;
pub const HEADER_BYTE_16: u8 = MAJOR_BYTE << 5 | SIZE_16;
pub const HEADER_BYTE_32: u8 = MAJOR_BYTE << 5 | SIZE_32;
pub const HEADER_BYTE_64: u8 = MAJOR_BYTE << 5 | SIZE_64;
pub const HEADER_BYTE_INFINITE: u8 = MAJOR_BYTE << 5 | SIZE_INFINITE;

pub const HEADER_TEXT_START: u8 = MAJOR_TEXT << 5;
pub const HEADER_TEXT_8: u8 = MAJOR_TEXT << 5 | SIZE_8;
pub const HEADER_TEXT_16: u8 = MAJOR_TEXT << 5 | SIZE_16;
pub const HEADER_TEXT_32: u8 = MAJOR_TEXT << 5 | SIZE_32;
pub const HEADER_TEXT_64: u8 = MAJOR_TEXT << 5 | SIZE_64;
pub const HEADER_TEXT_INFINITE: u8 = MAJOR_TEXT << 5 | SIZE_INFINITE;

pub const HEADER_ARRAY_START: u8 = MAJOR_ARRAY << 5;
pub const HEADER_ARRAY_8: u8 = MAJOR_ARRAY << 5 | SIZE_8;
pub const HEADER_ARRAY_16: u8 = MAJOR_ARRAY << 5 | SIZE_16;
pub const HEADER_ARRAY_32: u8 = MAJOR_ARRAY << 5 | SIZE_32;
pub const HEADER_ARRAY_64: u8 = MAJOR_ARRAY << 5 | SIZE_64;
pub const HEADER_ARRAY_INFINITE: u8 = MAJOR_ARRAY << 5 | SIZE_INFINITE;

pub const HEADER_MAP_START: u8 = MAJOR_MAP << 5;
pub const HEADER_MAP_8: u8 = MAJOR_MAP << 5 | SIZE_8;
pub const HEADER_MAP_16: u8 = MAJOR_MAP << 5 | SIZE_16;
pub const HEADER_MAP_32: u8 = MAJOR_MAP << 5 | SIZE_32;
pub const HEADER_MAP_64: u8 = MAJOR_MAP << 5 | SIZE_64;
pub const HEADER_MAP_INFINITE: u8 = MAJOR_MAP << 5 | SIZE_INFINITE;

pub const HEADER_TAG_START: u8 = MAJOR_TAG << 5;
pub const HEADER_TAG_8: u8 = MAJOR_TAG << 5 | SIZE_8;
pub const HEADER_TAG_16: u8 = MAJOR_TAG << 5 | SIZE_16;
pub const HEADER_TAG_32: u8 = MAJOR_TAG << 5 | SIZE_32;
pub const HEADER_TAG_64: u8 = MAJOR_TAG << 5 | SIZE_64;

pub const HEADER_FALSE: u8 = MAJOR_PRIMITIVE << 5 | 20;
pub const HEADER_TRUE: u8 = MAJOR_PRIMITIVE << 5 | 21;
pub const HEADER_NULL: u8 = MAJOR_PRIMITIVE << 5 | 22;
pub const HEADER_UNDEFINED: u8 = MAJOR_PRIMITIVE << 5 | 23;
pub const HEADER_BREAK: u8 = MAJOR_PRIMITIVE << 5 | 31;

pub const HEADER_FLOAT_16: u8 = MAJOR_PRIMITIVE << 5 | SIZE_16;
pub const HEADER_FLOAT_32: u8 = MAJOR_PRIMITIVE << 5 | SIZE_32;
pub const HEADER_FLOAT_64: u8 = MAJOR_PRIMITIVE << 5 | SIZE_64;
