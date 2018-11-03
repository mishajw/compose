//! Defines a tree of [`Player`](../core/trait.Player.html)s

use errors::*;

use std::collections::HashMap;

mod creation;
pub use self::creation::{create_continuous_input, create_player, FromSpec};

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
            Value::Int(_) => Err(ErrorKind::SpecTypeError(
                value_name.into(),
                "string".into(),
                "int".into(),
            ).into()),
            Value::Float(_) => Err(ErrorKind::SpecTypeError(
                value_name.into(),
                "string".into(),
                "float".into(),
            ).into()),
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
            Value::Str(_) => Err(ErrorKind::SpecTypeError(
                value_name.into(),
                "int".into(),
                "string".into(),
            ).into()),
            Value::Float(_) => Err(ErrorKind::SpecTypeError(
                value_name.into(),
                "int".into(),
                "float".into(),
            ).into()),
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
            Value::Str(_) => Err(ErrorKind::SpecTypeError(
                value_name.into(),
                "float".into(),
                "string".into(),
            ).into()),
            Value::Int(_) => Err(ErrorKind::SpecTypeError(
                value_name.into(),
                "float".into(),
                "int".into(),
            ).into()),
        }
    }

    /// Check that all values in the spec have been used
    pub fn ensure_all_used(&self) -> Result<()> {
        if self.values.is_empty() {
            Ok(())
        } else {
            Err(ErrorKind::SpecExtraValuesError(
                self.values.keys().map(|s| s.clone()).collect(),
            ).into())
        }
    }
}
