use core::spec::FromValue;
use core::spec::Spec;
use core::Consts;
use error::*;

/// Defines a field in a spec
pub struct FieldDeclaration<T: FromValue> {
    name: String,
    description: String,
    default_fn: Option<Box<Fn(&Consts) -> T + Sync + 'static>>,
}

impl<T: FromValue> FieldDeclaration<T> {
    #[allow(missing_docs)]
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self
    {
        Self {
            name: name.into(),
            description: description.into(),
            default_fn: None,
        }
    }

    #[allow(missing_docs)]
    pub fn with_default(
        name: impl Into<String>,
        description: impl Into<String>,
        default_fn: impl Fn(&Consts) -> T + Sync + 'static,
    ) -> Self
    {
        Self {
            name: name.into(),
            description: description.into(),
            default_fn: Some(Box::new(default_fn)),
        }
    }

    /// Get the description of the field
    pub fn to_description(&self) -> FieldDescription {
        FieldDescription {
            name: self.name.clone(),
            description: self.description.clone(),
            type_name: T::name().to_string(),
            has_default: self.default_fn.is_some(),
        }
    }

    /// Get the field's value from a spec
    pub fn get(&self, spec: &mut Spec, consts: &Consts) -> Result<T> {
        let result = match &self.default_fn {
            Some(default_fn) => spec.consume_with_default(
                &self.name,
                default_fn(consts),
                consts,
            ),
            None => spec.consume(&self.name, consts),
        };
        result.chain_err(|| format!("Failed to create field {}", self.name))
    }
}

/// Describes a field in the spec
///
/// Contains no compile-time type information, so that we can pass descriptions
/// around without worrying about generics
#[allow(unused)]
pub struct FieldDescription {
    name: String,
    description: String,
    type_name: String,
    has_default: bool,
}
