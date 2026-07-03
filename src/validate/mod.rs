pub mod constraint;
pub mod source;
pub mod structural;

pub use constraint::validate_constraints;
pub use source::validate_source;
pub use structural::{validate_structural, ValidationContext};

#[cfg(test)]
pub(crate) mod testutil {
    use super::ValidationContext;
    use crate::ast::Instance;
    use crate::error::Diagnostic;
    use std::path::Path;

    pub(crate) type Validator = fn(&ValidationContext, &Instance) -> Vec<Diagnostic>;

    /// Parse + resolve `src`, then run the given validation phases over every
    /// instance. Resolve diagnostics are dropped so each test targets only the
    /// phases it asks for.
    pub(crate) fn run_validators(src: &str, validators: &[Validator]) -> Vec<Diagnostic> {
        let path = Path::new("test.ilk");
        let file = crate::parser::parse(src, path).unwrap();
        let (env, _) = crate::resolve::resolve(&file, path);
        let ctx = ValidationContext::new(&env, path);
        let mut errors = Vec::new();
        for inst in file.instances() {
            for validate in validators {
                errors.extend(validate(&ctx, inst));
            }
        }
        errors
    }
}
