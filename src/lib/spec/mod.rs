//! Defines a tree of [`Player`](../core/trait.Player.html)s

use errors::*;

use std::collections::HashMap;

mod creation;
pub mod yaml;

pub use self::creation::{
    create_bool_input, create_bounded_input, create_outputs, create_player,
    FromSpec,
};

/// A key-value store for defining compositions
#[derive(Clone)]
pub struct Spec {
    values: HashMap<String, Value>,
}

/// A value in a [`Spec`](struct.Spec.html)
#[derive(Clone)]
pub enum Value {
    #[allow(missing_docs)]
    Str(String),
    #[allow(missing_docs)]
    Int(i32),
    #[allow(missing_docs)]
    Float(f32),
    #[allow(missing_docs)]
    Spec(Spec),
    #[allow(missing_docs)]
    Bool(bool),
    #[allow(missing_docs)]
    List(Vec<Value>),
}

impl Spec {
    #[allow(missing_docs)]
    pub fn new(values: HashMap<String, Value>) -> Self { Spec { values } }

    /// Get a value from the spec, and remove it
    pub fn consume<T: ValueType>(&mut self, value_name: &str) -> Result<T> {
        let value: Value = self
            .values
            .remove(value_name)
            .ok_or_else(|| ErrorKind::SpecMissingError(value_name.into()))?;
        T::get_from_value(value).ok_or_else(|| {
            ErrorKind::SpecTypeError(value_name.into(), "string".into()).into()
        })
    }

    /// Get a value from the spec, and remove it
    pub fn consume_with_default<T: ValueType>(
        &mut self,
        value_name: &str,
        default: T,
    ) -> Result<T>
    {
        match self.values.remove(value_name) {
            Some(value) => T::get_from_value(value).ok_or_else(|| {
                ErrorKind::SpecTypeError(value_name.into(), "string".into())
                    .into()
            }),
            None => Ok(default),
        }
    }

    /// Check that all values in the spec have been used
    pub fn ensure_all_used(&self) -> Result<()> {
        if self.values.is_empty() {
            Ok(())
        } else {
            Err(ErrorKind::SpecExtraValuesError(
                self.values.keys().cloned().collect(),
            )
            .into())
        }
    }
}

/// A type that can be extracted from a `Value`
pub trait ValueType: Sized {
    /// Get the type from the `Value`
    fn get_from_value(value: Value) -> Option<Self>;
}

macro_rules! impl_value_type {
    ($extracted_type:ty, $value_pattern:tt) => {
        impl ValueType for $extracted_type {
            fn get_from_value(value: Value) -> Option<Self> {
                match value {
                    Value::$value_pattern(extracted) => Some(extracted),
                    _ => None,
                }
            }
        }
    };
}

impl_value_type!(String, Str);
impl_value_type!(i32, Int);
impl_value_type!(f32, Float);
impl_value_type!(bool, Bool);
impl_value_type!(Spec, Spec);
impl_value_type!(Vec<Value>, List);

impl Value {
    /// Try convert the value into a `Spec`
    pub fn as_spec(self) -> Result<Spec> {
        if let Value::Spec(spec) = self {
            Ok(spec)
        } else {
            Err(ErrorKind::BadInput("Failed cast as spec".into()).into())
        }
    }
}
