//!
//! The contract ABI constants.
//!

/// The calldata size.
pub const OFFSET_CALLDATA_SIZE: usize = 0;

/// The return data size.
pub const OFFSET_RETURN_DATA_SIZE: usize = 1;

/// The entry data, e.g. the method hash and constructor bit.
pub const OFFSET_ENTRY_DATA: usize = 7;

/// The call and return data.
pub const OFFSET_CALL_RETURN_DATA: usize = 8;

/// The constructor entry mask.
pub const CONSTRUCTOR_ENTRY_MASK: u32 = 1;

/// The Solidity memory first hashing slot offset.
pub const OFFSET_SOLIDITY_HASH_SLOT_FIRST: usize = 0;

/// The Solidity memory second hashing slot offset.
pub const OFFSET_SOLIDITY_HASH_SLOT_SECOND: usize = 1;

/// The Solidity memory pointer offset.
pub const OFFSET_SOLIDITY_MEMORY_POINTER_SLOT: usize = 2;

/// The Solidity memory zero slot offset.
pub const OFFSET_SOLIDITY_ZERO_SLOT: usize = 3;

/// The constructor flag key preimage.
pub static CONSTRUCTOR_EXECUTED_FLAG_KEY_PREIMAGE: &str = "zkSyncEVM_ConstructorExecuted";

/// The `expected constructor call` error preimage.
pub static ERROR_EXPECTED_CONSTRUCTOR_CALL: &str = "zkSyncEVM_ErrorExpectedConstructorCall";

/// The `double constructor call` error preimage.
pub static ERROR_DOUBLE_CONSTRUCTOR_CALL: &str = "zkSyncEVM_ErrorDoubleConstructorCall";
