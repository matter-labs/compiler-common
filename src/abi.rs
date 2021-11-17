//!
//! The contract ABI constants.
//!

/// The header offset.
pub const OFFSET_HEADER: usize = 0;

/// The call and return data offset.
pub const OFFSET_DATA: usize = 1;

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

/// The `sent/transfer forbidden` error preimage.
pub static ERROR_FORBIDDEN_SEND_TRANSFER: &str = "zkSyncEVM_ErrorSendTransferForbidden";
