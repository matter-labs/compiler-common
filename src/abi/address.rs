//!
//! The zkEVM constant addresses.
//!

/// The bootloader formal address.
pub static BOOTLOADER: &str = "0x0000000000000000000000000000000000008001";

/// The account code storage system contract address.
pub static ACCOUNT_CODE_STORAGE: &str = "0x0000000000000000000000000000000000008002";

/// The nonce holder system contract address.
pub static NONCE_HOLDER: &str = "0x0000000000000000000000000000000000008003";

/// The known code factory system contract address.
pub static KNOWN_CODES_STORAGE: &str = "0x0000000000000000000000000000000000008004";

/// The immutable simulator system contract address.
pub static IMMUTABLE_SIMULATOR: &str = "0x0000000000000000000000000000000000008005";

/// The contract deployer system contract address.
pub static CONTRACT_DEPLOYER: &str = "0x0000000000000000000000000000000000008006";

/// The `msg.value` system contract address.
pub static MSG_VALUE: &str = "0x0000000000000000000000000000000000008007";

/// The `keccak256` system contract address.
pub static KECCAK256: &str = "0x0000000000000000000000000000000000008010";

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

/// The `system_call` simulation predefined address.
pub static SYSTEM_CALL: &str = "0x000000000000000000000000000000000000FFFA";

/// The `set_context_value_call` simulation predefined address.
pub static SET_CONTEXT_VALUE_CALL: &str = "0x000000000000000000000000000000000000FFF9";

/// The ETH token predefined address.
pub static ETH_TOKEN: &str = "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE";
