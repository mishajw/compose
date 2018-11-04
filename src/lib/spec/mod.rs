//! Defines a tree of [`Player`](../core/trait.Player.html)s

use errors::*;

use std::collections::HashMap;

mod creation;
pub mod yaml;

pub use self::creation::{
    create_continuous_input, create_outputs, create_player, FromSpec,
};

/// A key-value store for defining compositions
pub struct Spec {
    values: HashMap<String, Value>,
}

/// A value in a [`Spec`](struct.Spec.html)
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

    /// Get a string from the spec, and remove it
    pub fn use_str(&mut self, value_name: &str) -> Result<String> {
        let value: Value = self
            .values
            .remove(value_name)
            .ok_or_else(|| ErrorKind::SpecMissingError(value_name.into()))?;

        match value {
            Value::Str(string) => Ok(string),
            _ => Err(ErrorKind::SpecTypeError(
                value_name.into(),
                "string".into(),
            )
            .into()),
        }
    }

    /// Get a integer from the spec, and remove it
    pub fn use_int(&mut self, value_name: &str) -> Result<i32> {
        let value: Value = self
            .values
            .remove(value_name)
            .ok_or_else(|| ErrorKind::SpecMissingError(value_name.into()))?;

        match value {
            Value::Int(int) => Ok(int),
            _ => Err(ErrorKind::SpecTypeError(value_name.into(), "int".into())
                .into()),
        }
    }

    /// Get a integer from the spec, and remove it
    pub fn use_float(&mut self, value_name: &str) -> Result<f32> {
        let value: Value = self
            .values
            .remove(value_name)
            .ok_or_else(|| ErrorKind::SpecMissingError(value_name.into()))?;

        match value {
            Value::Float(float) => Ok(float),
            _ => {
                Err(ErrorKind::SpecTypeError(value_name.into(), "float".into())
                    .into())
            }
        }
    }

    /// Get a object from the spec, and remove it
    pub fn use_spec(&mut self, value_name: &str) -> Result<Spec> {
        let value: Value = self
            .values
            .remove(value_name)
            .ok_or_else(|| ErrorKind::SpecMissingError(value_name.into()))?;

        match value {
            Value::Spec(spec) => Ok(spec),
            _ => {
                Err(ErrorKind::SpecTypeError(value_name.into(), "spec".into())
                    .into())
            }
        }
    }

    /// Get a object from the spec, and remove it
    pub fn use_bool(&mut self, value_name: &str) -> Result<bool> {
        let value: Value = self
            .values
            .remove(value_name)
            .ok_or_else(|| ErrorKind::SpecMissingError(value_name.into()))?;

        match value {
            Value::Bool(bool) => Ok(bool),
            _ => {
                Err(ErrorKind::SpecTypeError(value_name.into(), "bool".into())
                    .into())
            }
        }
    }

    /// Get a object from the spec, and remove it
    pub fn use_list(&mut self, value_name: &str) -> Result<Vec<Value>> {
        let value: Value = self
            .values
            .remove(value_name)
            .ok_or_else(|| ErrorKind::SpecMissingError(value_name.into()))?;

        match value {
            Value::List(list) => Ok(list),
            _ => {
                Err(ErrorKind::SpecTypeError(value_name.into(), "list".into())
                    .into())
            }
        }
    }

    /// Check that all values in the spec have been used
    pub fn ensure_all_used(&self) -> Result<()> {
        if self.values.is_empty() {
            Ok(())
        } else {
            Err(ErrorKind::SpecExtraValuesError(
                self.values.keys().map(|s| s.clone()).collect(),
            )
            .into())
        }
    }
}
