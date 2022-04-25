//!
//! The zkEVM ABI memory constants.
//!

/// The heap size in cells.
pub const SIZE: usize = 0x80000;

/// The calldata offset's offset (cells).
pub const OFFSET_CALLDATA_OFFSET: usize = SIZE - 1;

/// The calldata length's offset (cells).
pub const OFFSET_CALLDATA_LENGTH: usize = SIZE - 2;

/// The return data offset's offset (cells).
pub const OFFSET_RETURN_DATA_OFFSET: usize = SIZE - 3;

/// The return data length's offset (cells).
pub const OFFSET_RETURN_DATA_LENGTH: usize = SIZE - 4;
