//!
//! The compiler common library.
//!

pub(crate) mod abi;
pub(crate) mod base;
pub(crate) mod bitlength;
pub(crate) mod exit_code;
pub(crate) mod extension;
pub(crate) mod hashes;
pub(crate) mod size;
pub(crate) mod solidity;
pub(crate) mod vyper;

pub use self::abi::address::BOOTLOADER as ABI_ADDRESS_BOOTLOADER;
pub use self::abi::address::CODE_ADDRESS as ABI_ADDRESS_CODE_ADDRESS;
pub use self::abi::address::DEPLOYER as ABI_ADDRESS_DEPLOYER;
pub use self::abi::address::ECRECOVER as ABI_ADDRESS_ECRECOVER;
pub use self::abi::address::KECCAK256 as ABI_ADDRESS_KECCAK256;
pub use self::abi::address::KNOWN_CODE_FACTORY as ABI_ADDRESS_KNOWN_CODE_FACTORY;
pub use self::abi::address::META as ABI_ADDRESS_META;
pub use self::abi::address::MIMIC_CALL as ABI_ADDRESS_MIMIC_CALL;
pub use self::abi::address::NONCE_MANAGER as ABI_ADDRESS_NONCE_MANAGER;
pub use self::abi::address::PRECOMPILE as ABI_ADDRESS_PRECOMPILE;
pub use self::abi::address::SHA256 as ABI_ADDRESS_SHA256;
pub use self::abi::address::TO_L1 as ABI_ADDRESS_TO_L1;
pub use self::abi::error::FORBIDDEN_SEND_TRANSFER as ABI_ERROR_FORBIDDEN_SEND_TRANSFER;
pub use self::abi::memory::OFFSET_CALLDATA_LENGTH as ABI_MEMORY_OFFSET_CALLDATA_LENGTH;
pub use self::abi::memory::OFFSET_CALLDATA_OFFSET as ABI_MEMORY_OFFSET_CALLDATA_OFFSET;
pub use self::abi::memory::OFFSET_RETURN_DATA_LENGTH as ABI_MEMORY_OFFSET_RETURN_DATA_LENGTH;
pub use self::abi::memory::OFFSET_RETURN_DATA_OFFSET as ABI_MEMORY_OFFSET_RETURN_DATA_OFFSET;
pub use self::abi::memory::SIZE as ABI_MEMORY_SIZE;
pub use self::base::BINARY as BASE_BINARY;
pub use self::base::DECIMAL as BASE_DECIMAL;
pub use self::base::HEXADECIMAL as BASE_HEXADECIMAL;
pub use self::base::OCTAL as BASE_OCTAL;
pub use self::bitlength::BOOLEAN as BITLENGTH_BOOLEAN;
pub use self::bitlength::BYTE as BITLENGTH_BYTE;
pub use self::bitlength::ETH_ADDRESS as BITLENGTH_ETH_ADDRESS;
pub use self::bitlength::FIELD as BITLENGTH_FIELD;
pub use self::bitlength::X32 as BITLENGTH_X32;
pub use self::bitlength::X64 as BITLENGTH_X64;
pub use self::bitlength::ZKEVM_POINTER as BITLENGTH_ZKEVM_POINTER;
pub use self::exit_code::FAILURE as EXIT_CODE_FAILURE;
pub use self::exit_code::SUCCESS as EXIT_CODE_SUCCESS;
pub use self::extension::EVM as EXTENSION_EVM;
pub use self::extension::EVM_BINARY as EXTENSION_EVM_BINARY;
pub use self::extension::JSON as EXTENSION_JSON;
pub use self::extension::LLVM_BINARY as EXTENSION_LLVM_BINARY;
pub use self::extension::LLVM_SOURCE as EXTENSION_LLVM_SOURCE;
pub use self::extension::MANIFEST as EXTENSION_MANIFEST;
pub use self::extension::SOLIDITY as EXTENSION_SOLIDITY;
pub use self::extension::VYPER as EXTENSION_VYPER;
pub use self::extension::YUL as EXTENSION_YUL;
pub use self::extension::ZINC as EXTENSION_ZINC;
pub use self::extension::ZKEVM_ASSEMBLY as EXTENSION_ZKEVM_ASSEMBLY;
pub use self::extension::ZKEVM_BINARY as EXTENSION_ZKEVM_BINARY;
pub use self::hashes::keccak256;
pub use self::size::BYTE as SIZE_BYTE;
pub use self::size::ETH_ADDRESS as SIZE_ETH_ADDRESS;
pub use self::size::FIELD as SIZE_FIELD;
pub use self::size::X32 as SIZE_X32;
pub use self::size::X64 as SIZE_X64;
pub use self::size::ZKEVM_POINTER as SIZE_ZKEVM_POINTER;
pub use self::solidity::address::ECRECOVER as SOLIDITY_ADDRESS_ECRECOVER;
pub use self::solidity::address::IDENTITY as SOLIDITY_ADDRESS_IDENTITY;
pub use self::solidity::address::RIPEMD160 as SOLIDITY_ADDRESS_RIPEMD160;
pub use self::solidity::address::SHA256 as SOLIDITY_ADDRESS_SHA256;
pub use self::solidity::memory::OFFSET_EMPTY_SLOT as SOLIDITY_MEMORY_OFFSET_EMPTY_SLOT;
pub use self::solidity::memory::OFFSET_MEMORY_POINTER as SOLIDITY_MEMORY_OFFSET_MEMORY_POINTER;
pub use self::solidity::memory::OFFSET_NON_RESERVED as SOLIDITY_MEMORY_OFFSET_NON_RESERVED;
pub use self::solidity::memory::OFFSET_SCRATCH_SPACE as SOLIDITY_MEMORY_OFFSET_SCRATCH_SPACE;
pub use self::vyper::forwarder::BYTECODE as VYPER_FORWARDER_BYTECODE;
pub use self::vyper::memory::OFFSET_FREE_VAR_SPACE as VYPER_MEMORY_OFFSET_FREE_VAR_SPACE;
pub use self::vyper::memory::OFFSET_FREE_VAR_SPACE2 as VYPER_MEMORY_OFFSET_FREE_VAR_SPACE2;
pub use self::vyper::memory::OFFSET_NON_RESERVED as VYPER_MEMORY_OFFSET_NON_RESERVED;
