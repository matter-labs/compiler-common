//!
//! The zkEVM ABI memory constants.
//!

/// The heap size in cells.
pub const SIZE: usize = 0xffff;

/// The calldata and return data offset's offset (cells).
pub const OFFSET_DATA_OFFSET: usize = SIZE - 1;

/// The calldata and return data length's offset (cells).
pub const OFFSET_DATA_LENGTH: usize = SIZE - 2;
