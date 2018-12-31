//! Defines a tree of [`Player`](../core/trait.Player.html)s

use error::*;

use std::collections::HashMap;

pub mod create;
mod spec_macro;
pub mod yaml;

pub use self::spec_macro::SpecMacro;

/// A key-value store for defining compositions
#[derive(Clone, Debug)]
pub struct Spec {
    values: HashMap<String, Value>,
}

/// A value in a [`Spec`](struct.Spec.html)
#[derive(Clone, Debug)]
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

    #[allow(missing_docs)]
    pub fn empty() -> Self {
        Spec {
            values: HashMap::new(),
        }
    }

    /// Get a value from the spec, and remove it
    pub fn consume<T: ValueType>(&mut self, value_name: &str) -> Result<T> {
        let value: Value = self
            .values
            .remove(value_name)
            .ok_or_else(|| ErrorKind::SpecMissingError(value_name.into()))?;
        T::get_from_value(value).ok_or_else(|| {
            ErrorKind::SpecTypeError(
                value_name.into(),
                T::get_type_name().into(),
            )
            .into()
        })
    }

    /// Get a value from the spec, and remove it. If it doesn't exist, return
    /// `default`
    pub fn consume_with_default<T: ValueType>(
        &mut self,
        value_name: &str,
        default: T,
    ) -> Result<T>
    {
        match self.values.remove(value_name) {
            Some(value) => T::get_from_value(value).ok_or_else(|| {
                ErrorKind::SpecTypeError(
                    value_name.into(),
                    T::get_type_name().into(),
                )
                .into()
            }),
            None => Ok(default),
        }
    }

    /// Get a value from the spec, and remove it. If it doesn't exist, return
    /// `None`
    pub fn consume_optional<T: ValueType>(
        &mut self,
        value_name: &str,
    ) -> Result<Option<T>>
    {
        match self.values.remove(value_name) {
            Some(value) => T::get_from_value(value)
                .ok_or_else(|| {
                    ErrorKind::SpecTypeError(
                        value_name.into(),
                        T::get_type_name().into(),
                    )
                    .into()
                })
                .map(Some),
            None => Ok(None),
        }
    }

    /// Consume a list of elements as a type
    pub fn consume_list<T: ValueType>(
        &mut self,
        list_name: &str,
    ) -> Result<Vec<T>>
    {
        let value_list: Vec<Value> =
            if let Some(value_list) = self.consume_optional(list_name)? {
                value_list
            } else {
                // If the field doesn't exist, return an empty list
                return Ok(vec![]);
            };

        value_list
            .into_iter()
            .map(|v| v.as_type::<T>())
            .collect::<Result<Vec<_>>>()
    }

    /// Add a field to the spec, returning the modified spec
    pub fn with<T: ValueType>(mut self, value_name: String, value: T) -> Spec {
        self.values.insert(value_name, value.into_value());
        self
    }

    /// Add a field to the spec
    pub fn put<T: ValueType>(&mut self, value_name: String, value: T) {
        self.values.insert(value_name, value.into_value());
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

    /// Get the names of the values in the spec
    pub fn value_names(&self) -> Vec<String> {
        self.values.keys().cloned().collect()
    }
}

/// A type that can be extracted from a `Value`
pub trait ValueType: Sized {
    /// Get the name of the type for error messages
    fn get_type_name() -> &'static str;

    /// Get the type from the `Value`
    fn get_from_value(value: Value) -> Option<Self>;

    /// Get the type from the `Value`
    fn into_value(self) -> Value;
}

macro_rules! impl_value_type {
    ($extracted_type:ty, $value_pattern:tt) => {
        impl ValueType for $extracted_type {
            fn get_type_name() -> &'static str { stringify!($extracted_type) }

            fn get_from_value(value: Value) -> Option<Self> {
                match value {
                    Value::$value_pattern(extracted) => Some(extracted),
                    _ => None,
                }
            }

            fn into_value(self) -> Value { Value::$value_pattern(self) }
        }
    };
}

impl_value_type!(String, Str);
impl_value_type!(i32, Int);
impl_value_type!(f32, Float);
impl_value_type!(bool, Bool);
impl_value_type!(Spec, Spec);
impl_value_type!(Vec<Value>, List);

impl ValueType for Value {
    fn get_type_name() -> &'static str { "Value" }

    fn get_from_value(value: Value) -> Option<Self> { Some(value) }

    fn into_value(self) -> Value { self }
}

impl Value {
    /// Try return the value as a type
    pub fn as_type<T: ValueType>(self) -> Result<T> {
        T::get_from_value(self).ok_or_else(|| {
            ErrorKind::BadInput("Failed cast as spec".into()).into()
        })
    }
}
