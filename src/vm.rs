//!
//! The virtual machine tools.
//!

/// The LLVM target name.
pub static TARGET_NAME: &str = "syncvm";

/// The actual production VM name.
pub static PRODUCTION_NAME: &str = "zkEVM";

///
/// Initializes the zkEVM target machine.
///
pub fn initialize_target() {
    inkwell::targets::Target::initialize_syncvm(&inkwell::targets::InitializationConfig::default());
}

///
/// Returns the zkEVM target machine instance.
///
pub fn target_machine(
    optimization_level: inkwell::OptimizationLevel,
) -> Option<inkwell::targets::TargetMachine> {
    inkwell::targets::Target::from_name(TARGET_NAME)?.create_target_machine(
        &inkwell::targets::TargetTriple::create(TARGET_NAME),
        "",
        "",
        optimization_level,
        inkwell::targets::RelocMode::Default,
        inkwell::targets::CodeModel::Default,
    )
}
