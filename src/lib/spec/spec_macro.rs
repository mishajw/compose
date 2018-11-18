use errors::*;
use spec::Spec;
use spec::Value;

/// Resolvable macro in the spec
pub trait SpecMacro {
    /// The name of the macro, checked when invoking
    fn name() -> &'static str;

    /// Resolve the macro on a spec
    fn resolve(spec: &mut Spec) -> Result<Value>;
}
