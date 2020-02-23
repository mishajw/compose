use core::spec::FromValue;
use core::spec::Spec;
use core::Consts;
use error::*;

/// Defines a field in a spec
pub struct SpecField<T: FromValue> {
    name: String,
    description: String,
    default_fn: Option<Box<dyn Fn(&Consts) -> T + Sync + 'static>>,
}

impl<T: FromValue> SpecField<T> {
    #[allow(missing_docs)]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
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
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            default_fn: Some(Box::new(default_fn)),
        }
    }

    /// Get the description of the field
    pub fn to_description(&self) -> SpecFieldDescription {
        SpecFieldDescription {
            name: self.name.clone(),
            description: self.description.clone(),
            type_name: T::name(),
            has_default: self.default_fn.is_some(),
        }
    }

    /// Get the field's value from a spec
    pub fn get(&self, spec: &mut Spec, consts: &Consts) -> Result<T> {
        let result = match &self.default_fn {
            Some(default_fn) => spec.consume_with_default(&self.name, default_fn(consts), consts),
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
#[derive(Clone)]
pub struct SpecFieldDescription {
    /// The name of the field
    pub name: String,
    /// Description of the field
    pub description: String,
    /// The type of the field
    pub type_name: String,
    /// True if the field has a default value
    pub has_default: bool,
}

/// Macro to define fields easier
macro_rules! field_decl {
    (
        $field_name:ident,
        $field_type:ty,
        $description:expr
    ) => {
        lazy_static! {
            static ref $field_name: SpecField<$field_type> = SpecField::new(
                stringify!($field_name).to_lowercase().replace("_", "-"),
                $description
            );
        }
    };
    (
        $field_name:ident,
        $field_type:ty,
        $description:expr,
        $default_fn:expr
    ) => {
        lazy_static! {
            static ref $field_name: SpecField<$field_type> = SpecField::with_default(
                stringify!($field_name).to_lowercase().replace("_", "-"),
                $description,
                $default_fn,
            );
        }
    };
}
