//!
//! The zkEVM constant addresses.
//!

/// The `ecrecover` predefined address.
pub static ECRECOVER: &str = "0x0000000000000000000000000000000000000001";

/// The `sha256` predefined address.
pub static SHA256: &str = "0x0000000000000000000000000000000000000002";

/// The `ripemd160` predefined address.
pub static RIPEMD160: &str = "0x0000000000000000000000000000000000000003";

/// The `identity` predefined address.
pub static IDENTITY: &str = "0x0000000000000000000000000000000000000004";

/// The `keccak256` predefined address.
pub static KECCAK256: &str = "0x00000000000000000000000000000000000003FF";

/// The `create[2]` predefined address.
pub static CREATE: &str = "0x00000000000000000000000000000000000003FE";

/// The `to_l1` simulation predefined address.
pub static TO_L1: &str = "0x000000000000000000000000000000000000FFFF";

/// The `code_address` simulation predefined address.
pub static CODE_ADDRESS: &str = "0x000000000000000000000000000000000000FFFE";

/// The `precompile` simulation predefined address.
pub static PRECOMPILE: &str = "0x000000000000000000000000000000000000FFFD";

/// The `meta` simulation predefined address.
pub static META: &str = "0x000000000000000000000000000000000000FFFC";

/// The `mimic_call` simulation predefined address.
pub static MIMIC_CALL: &str = "0x000000000000000000000000000000000000FFFB";
