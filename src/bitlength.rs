//!
//! The integer bitlength constants.
//!

/// The `bool` type bitlength.
pub const BOOLEAN: usize = 1;

/// The `u8` type or byte bitlength.
pub const BYTE: usize = 8;

/// The zkEVM pointer type (usually `16`) bitlength.
pub const ZKEVM_POINTER: usize = crate::size::ZKEVM_POINTER * BYTE;

/// The x86 word type (usually `u32`) bitlength.
pub const X32: usize = crate::size::X32 * BYTE;

/// The x86_64 word type (usually `u64`) bitlength.
pub const X64: usize = crate::size::X64 * BYTE;

/// The ETH address (usually `u160`) bitlength.
pub const ETH_ADDRESS: usize = crate::size::ETH_ADDRESS * BYTE;

/// The zkEVM field (usually `u256` or `i256`) bitlength.
pub const FIELD: usize = crate::size::FIELD * BYTE;
