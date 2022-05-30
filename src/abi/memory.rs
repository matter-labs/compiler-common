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

/// The temporary storage of the data replaced by `msg.value` of the ABI calldata (cells).
pub const OFFSET_ABI_VALUE_TEMPORARY_DATA: usize = SIZE - 5;

/// The temporary storage of the data replaced by `address` of the ABI calldata (cells).
pub const OFFSET_ABI_ADDRESS_TEMPORARY_DATA: usize = SIZE - 6;

/// The Vyper forwarder calldata space offset (cells).
pub const OFFSET_VYPER_FORWARDER_CALLDATA: usize = SIZE - 64;

/// The constructor return data space offset (cells).
pub const OFFSET_CONSTRUCTOR_RETURN_DATA: usize = SIZE - 0xffff;
