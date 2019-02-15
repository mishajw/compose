use core::spec::Spec;
use core::spec::Value;
use core::Consts;
use error::*;

/// A type that can be extracted from a `Value`
pub trait FromValue<CreatedType = Self>: Sized {
    /// Get the name of the type for error messages
    fn name() -> String;

    /// Create the type from a `Value`
    fn from_value(value: Value, consts: &Consts) -> Result<CreatedType>;
}

/// A primitive type that can be extracted from a `Value`
pub trait FromPrimitiveValue: FromValue {
    /// Get a reference to the type from the `Value`, returns `None` if
    /// incorrect value
    fn from_value_opt(value: &Value) -> Option<&Self>;

    /// Get a mutable reference to the type from the `Value`, returns `None` if
    /// incorrect value
    fn from_value_mut(value: &mut Value) -> Option<&mut Self>;

    /// Get a `Value` from the type
    fn into_value(self) -> Value;
}

macro_rules! from_primitive_value_impl {
    ($extracted_type:ty, $value_pattern:tt) => {
        impl FromValue for $extracted_type {
            fn name() -> String { stringify!($extracted_type).into() }

            fn from_value(value: Value, _consts: &Consts) -> Result<Self> {
                match value {
                    Value::$value_pattern(extracted) => Ok(extracted),
                    value => Err(ErrorKind::SpecError(format!(
                        "Incorrect type, expected {}, got {:?}",
                        Self::name().to_string(),
                        value
                    ))
                    .into()),
                }
            }
        }
        impl FromPrimitiveValue for $extracted_type {
            fn from_value_opt(value: &Value) -> Option<&Self> {
                match value {
                    Value::$value_pattern(extracted) => Some(extracted),
                    _ => None,
                }
            }

            fn from_value_mut(value: &mut Value) -> Option<&mut Self> {
                match value {
                    Value::$value_pattern(extracted) => Some(extracted),
                    _ => None,
                }
            }

            fn into_value(self) -> Value { Value::$value_pattern(self) }
        }
    };
}

from_primitive_value_impl!(String, Str);
from_primitive_value_impl!(i32, Int);
from_primitive_value_impl!(f64, Float);
from_primitive_value_impl!(bool, Bool);
// TODO: Do we want this implementation?
from_primitive_value_impl!(Spec, Spec);

impl FromValue for Value {
    fn name() -> String { "value".into() }

    fn from_value(value: Value, _consts: &Consts) -> Result<Self> { Ok(value) }
}

impl FromPrimitiveValue for Value {
    fn from_value_opt(value: &Value) -> Option<&Self> { Some(value) }

    fn from_value_mut(value: &mut Value) -> Option<&mut Self> { Some(value) }

    fn into_value(self) -> Value { self }
}

impl<T: FromValue> FromValue for Vec<T> {
    fn name() -> String { T::name() + "[]" }

    fn from_value(value: Value, consts: &Consts) -> Result<Vec<T>> {
        if let Value::List(list) = value {
            list.into_iter()
                .map(|v| T::from_value(v, consts))
                .collect::<Result<_>>()
        } else {
            bail!(ErrorKind::SpecError("Expected list type".into()));
        }
    }
}
