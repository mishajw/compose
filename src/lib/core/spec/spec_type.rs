use core::spec::FromValue;
use core::spec::Spec;
use core::spec::SpecFieldDescription;
use core::spec::Value;
use core::Consts;
use error::*;

/// Implementors can be created from a spec
pub trait SpecType<CreatedType = Self>: Sized {
    /// Get the name of the type
    fn name() -> &'static str;

    /// Get a list of field descriptions for the spec
    fn field_descriptions() -> Vec<SpecFieldDescription>;

    /// Create the value from the spec
    fn from_spec(spec: Spec, consts: &Consts) -> Result<CreatedType>;
}

impl<S: SpecType<T>, T> FromValue<T> for S {
    fn name() -> &'static str { <Self as SpecType<T>>::name() }

    fn from_value(value: Value, consts: &Consts) -> Result<T> {
        let spec: Spec = value.into_type(consts)?;
        Self::from_spec(spec, consts)
    }
}
