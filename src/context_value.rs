//!
//! The contract context values.
//!

///
/// The contract context values.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextValue {
    /// The current contract address.
    Address,
    /// The caller/parent address.
    Caller,
    /// The contract where the address is deployed.
    CodeAddress,
    /// The meta value.
    Meta,
    /// The transaction origin value.
    TxOrigin,
    /// The remaining amount of ergs.
    ErgsLeft,
    /// The current stack pointer value.
    StackPointer,
}

impl From<ContextValue> for u64 {
    fn from(value: ContextValue) -> Self {
        match value {
            ContextValue::Address => 0,
            ContextValue::Caller => 1,
            ContextValue::CodeAddress => 2,
            ContextValue::Meta => 3,
            ContextValue::TxOrigin => 4,
            ContextValue::ErgsLeft => 5,
            ContextValue::StackPointer => 6,
        }
    }
}
