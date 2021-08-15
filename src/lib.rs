//!
//! The compiler common library.
//!

pub(crate) mod address_space;
pub(crate) mod context_value;

pub mod abi;
pub mod base;
pub mod bitlength;
pub mod exit_code;
pub mod extension;
pub mod file_name;
pub mod identifier;
pub mod size;
pub mod vm;

pub use self::address_space::AddressSpace;
pub use self::context_value::ContextValue;
