//!
//! The common LLVM identifiers.
//!

/// The contract entry selector function name.
pub static FUNCTION_SELECTOR: &str = "__selector";

/// The contract constructor function name.
pub static FUNCTION_CONSTRUCTOR: &str = "__constructor";

/// The LLVM personality function name.
pub static FUNCTION_PERSONALITY: &str = "__personality";

/// The LLVM exception throwing function name.
pub static FUNCTION_CXA_THROW: &str = "__cxa_throw";

/// The `addmod` runtime function name.
pub static FUNCTION_ADDMOD: &str = "__addmod";

/// The `mulmod` runtime function name.
pub static FUNCTION_MULMOD: &str = "__mulmod";
