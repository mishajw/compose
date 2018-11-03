//! Defines a tree of [`Player`](../core/trait.Player.html)s

use core::input;
use core::Player;
use errors::*;

use std::collections::HashMap;

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

/// Implementors can be created from a spec
pub trait FromSpec {
    /// The name of the value to be created, used to find the type of the
    /// definition
    fn name() -> &'static str;
    /// Create the value from a spec
    fn from_spec(spec: &mut Spec) -> Result<Box<Self>>;
}

#[allow(unused)]
fn create_with_type<T: FromSpec>(
    name: &str,
    spec: &mut Spec,
) -> Result<Option<Box<T>>>
{
    if name == T::name() {
        T::from_spec(spec).map(Some)
    } else {
        Ok(None)
    }
}

/// Create a player from the spec. Every creatable player has to be added to
/// this function
pub fn create_player(spec: &mut Spec) -> Result<Box<Player>> {
    #[allow(unused)]
    let name = spec.use_str("name")?;
    let player: Option<Box<Player>> = None;
    player.ok_or_else(|| ErrorKind::SpecUnknownName(name).into())
}

/// Create an input from the spec. Every creatable input has to be added to
/// this function
pub fn create_continuous_input(
    spec: &mut Spec,
) -> Result<Box<input::Continuous>> {
    #[allow(unused)]
    let name = spec.use_str("name")?;
    let player: Option<Box<input::Continuous>> = None;
    player.ok_or_else(|| ErrorKind::SpecUnknownName(name).into())
}
