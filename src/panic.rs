//!
//! The Zinc panic constant messages.
//!

/// The builder pattern entity must be provided with the specified value.
pub static BUILDER_REQUIRES_VALUE: &str = "The builder requires a value: ";

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
