use core::spec::Spec;
use core::spec::Value;
use core::Consts;
use error::*;

/// A type that can be extracted from a `Value`
pub trait FromValue<CreatedType = Self>: Sized {
    /// Get the name of the type for error messages
    fn name() -> &'static str;

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
            fn name() -> &'static str { stringify!($extracted_type) }

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
from_primitive_value_impl!(Vec<Value>, List);
// TODO: Do we want this implementation?
from_primitive_value_impl!(Spec, Spec);

impl FromValue for Value {
    fn name() -> &'static str { "value" }

    fn from_value(value: Value, _consts: &Consts) -> Result<Self> { Ok(value) }
}

impl FromPrimitiveValue for Value {
    fn from_value_opt(value: &Value) -> Option<&Self> { Some(value) }

    fn from_value_mut(value: &mut Value) -> Option<&mut Self> { Some(value) }

    fn into_value(self) -> Value { self }
}

macro_rules! impl_from_value_switch {
    ($super_type:ty, $super_name:expr, $($sub_types:ty),*) => {
        use core::spec::{Value, FromValue, Spec};
        use core::Consts;
        use error::*;
        use std::collections::HashMap;
        type GetFn =
            Box<Fn(Value, &Consts) -> Result<Box<$super_type>> + Send + Sync>;
        lazy_static! {
            static ref MAP: HashMap<String, GetFn> = {
                let mut map = HashMap::new();
                $(
                    let create_fn: GetFn = Box::new(|v: Value, c: &Consts| {
                        let created = <$sub_types>::from_value(v, c);
                        created.map(|v| -> Box<$super_type> { Box::new(v) })
                    });
                    map.insert(<$sub_types>::name().to_string(), create_fn);
                )*
                map
            };
        }

        impl FromValue for Box<$super_type> {
            fn name() -> &'static str { $super_name }

            fn from_value(value: Value, consts: &Consts) -> Result<Self> {
                let mut spec: Spec = value.into_type(consts)?;
                let name: String = spec.consume("name", consts)
                    .chain_err(|| format!(
                        "Couldn't find name for {}", Self::name()
                    ))?;
                match MAP.get(&name) {
                    Some(create_fn) => {
                        create_fn(Value::Spec(spec), consts)
                            .chain_err(|| format!("Failed to create {}", name))
                    },
                    None => bail!(ErrorKind::SpecError(format!(
                        "Unrecognized name: {}", name
                    )))
                }
            }
        }
    };
}
