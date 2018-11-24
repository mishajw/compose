use core::spec::{Spec, Value};
use core::CompositionConsts;
use errors::*;

/// Resolvable macro in the spec
pub trait SpecMacro {
    /// The name of the macro, checked when invoking
    fn name() -> &'static str;

    /// Resolve the macro on a spec
    fn resolve(spec: &mut Spec, consts: &CompositionConsts) -> Result<Value>;
}
