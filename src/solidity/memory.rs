//!
//! The Solidity memory constants.
//!

/// The `keccak256` scratch space offset (cells).
pub const OFFSET_SCRATCH_SPACE: usize = 0;

/// The memory pointer offset (cells).
pub const OFFSET_MEMORY_POINTER: usize = 2;

/// The empty slot offset (cells).
pub const OFFSET_EMPTY_SLOT: usize = 3;

/// The unallocated space offset (cells).
pub const OFFSET_UNALLOCATED_SPACE: usize = 4;
