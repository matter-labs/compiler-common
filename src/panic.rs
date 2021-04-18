//!
//! The Zinc panic constant messages.
//!

/// The data conversion is always valid.
pub static DATA_CONVERSION: &str = "Data conversion is always successful: ";

/// The `Result` or `Option` value is always set. Should be eliminated where possible.
pub static VALUE_ALWAYS_EXISTS: &str = "Value always exists";

/// The builder pattern entity must be provided with the specified value.
pub static BUILDER_REQUIRES_VALUE: &str = "The builder requires a value: ";

/// The unit test data validity is checked by the test authors.
pub static TEST_DATA_VALID: &str = "Test data is always valid";

/// The source code mapping compiler phase responsibility.
pub static VALIDATED_DURING_SOURCE_CODE_MAPPING: &str = "Validated during source code mapping";

/// The lexical analysis compiler phase responsibility.
pub static VALIDATED_DURING_LEXICAL_ANALYSIS: &str = "Validated during lexical analysis";

/// The syntax analysis compiler phase responsibility.
pub static VALIDATED_DURING_SYNTAX_ANALYSIS: &str = "Validated during syntax analysis";

/// The semantic analysis compiler phase responsibility.
pub static VALIDATED_DURING_SEMANTIC_ANALYSIS: &str = "Validated during semantic analysis";

/// The target code generation compiler phase responsibility.
pub static VALIDATED_DURING_CODE_GENERATION: &str = "Validated during target code generation";

/// The virtual machine runtime execution responsibility.
pub static VALIDATED_DURING_RUNTIME_EXECUTION: &str = "Validated during runtime execution";

/// The Zandbox database integrity responsibility.
pub static VALIDATED_DURING_DATABASE_POPULATION: &str = "Validated during database population";
