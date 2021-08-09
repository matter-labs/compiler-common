//!
//! The compiler common library.
//!

pub(crate) mod address_space;
pub(crate) mod context_value;

pub mod app_name;
pub mod base;
pub mod bitlength;
pub mod contract;
pub mod exit_code;
pub mod extension;
pub mod file_name;
pub mod identifier;
pub mod panic;
pub mod size;
pub mod virtual_machine;

pub use self::address_space::AddressSpace;
pub use self::context_value::ContextValue;
