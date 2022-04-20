//!
//! The zkEVM constant addresses.
//!

/// The bootloader formal address.
pub static BOOTLOADER: &str = "0x0000000000000000000000000000000000000001";

/// The deployer system contract address.
pub static DEPLOYER: &str = "0x0000000000000000000000000000000000000002";

/// The nonce manager system contract address.
pub static NONCE_MANAGER: &str = "0x0000000000000000000000000000000000000003";

/// The known code factory system contract address.
pub static KNOWN_CODE_FACTORY: &str = "0x0000000000000000000000000000000000000004";

/// The `keccak256` predefined address.
pub static KECCAK256: &str = "0x0000000000000000000000000000000000000010";

/// The `sha256` predefined address.
pub static SHA256: &str = "0x0000000000000000000000000000000000000011";

/// The `ecrecover` predefined address.
pub static ECRECOVER: &str = "0x0000000000000000000000000000000000000012";

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
