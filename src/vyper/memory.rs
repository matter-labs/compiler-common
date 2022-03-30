//!
//! The Vyper memory constants.
//!

/// The `FREE_VAR_SPACE` offset (bytes).
pub const OFFSET_FREE_VAR_SPACE: usize = 0;

/// The `FREE_VAR_SPACE2` offset (bytes).
pub const OFFSET_FREE_VAR_SPACE2: usize = 32;

/// The non-reserved memory offset (bytes).
pub const OFFSET_NON_RESERVED: usize = 64;

/// The long-return flag memory offset (bytes).
pub const OFFSET_LONG_RETURN: usize = 0xffff * crate::size::FIELD;
