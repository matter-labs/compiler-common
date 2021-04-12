//!
//! The integer bitlength constants.
//!

/// The `bool` type bitlength.
pub const BOOLEAN: usize = 1;

/// The `u8` type or byte bitlength.
pub const BYTE: usize = 8;

/// The word type (usually `u64`) bitlength.
pub const WORD: usize = 64;

/// The `u256` or `i256` types bitlength.
pub const FIELD: usize = 256;

/// The ETH address bitlength.
pub const ETH_ADDRESS: usize = crate::size::ETH_ADDRESS * BYTE;
