//!
//! The smart contract constants.
//!

/// The implicit fields count.
pub const IMPLICIT_FIELDS_COUNT: usize = 2;

/// The first default implicit field index.
pub const FIELD_INDEX_ADDRESS: usize = 0;

/// The first default implicit field name.
pub static FIELD_NAME_ADDRESS: &str = "address";

/// The second default implicit field index.
pub const FIELD_INDEX_BALANCES: usize = 1;

/// The second default implicit field name.
pub static FIELD_NAME_BALANCES: &str = "balances";

/// The implicit transaction variable name.
pub static TRANSACTION_VARIABLE_NAME: &str = "msg";
