//!
//! The smart contract constants.
//!

/// The calldata size offset in the ABI.
pub const ABI_OFFSET_CALLDATA_SIZE: usize = 0;

/// The return data size offset in the ABI.
pub const ABI_OFFSET_RETURN_DATA_SIZE: usize = 1;

/// The entry function signature `keccak256` hash offset in the ABI.
pub const ABI_OFFSET_ENTRY_HASH: usize = 7;

/// The call and return data offset in the ABI.
pub const ABI_OFFSET_CALL_RETURN_DATA: usize = 8;
