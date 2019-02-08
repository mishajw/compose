//! Defines a tree of [`Player`](../core/trait.Player.html)s

use core::Consts;
use error::*;

use std::collections::HashMap;

mod field_declaration;
mod from_spec;
mod spec_macro;
#[macro_use]
mod from_value;
pub mod yaml;

pub use self::field_declaration::{FieldDeclaration, FieldDescription};
pub use self::from_spec::FromSpec;
pub use self::from_value::{FromPrimitiveValue, FromValue};
pub use self::spec_macro::{
    resolve_root_macros, resolve_spec_value, SpecMacro,
};

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
    Float(f64),
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

    /// Get a reference to a value in the spec
    pub fn get<'a, T: FromPrimitiveValue>(
        &'a self,
        value_name: &str,
    ) -> Result<&'a T>
    {
        let value: &'a Value =
            self.values.get(value_name).ok_or_else(|| {
                ErrorKind::SpecError(format!("Missing value: {}", value_name))
            })?;
        T::from_value_opt(value).ok_or_else(|| {
            ErrorKind::SpecError(format!(
                "Incorrect type, expected: {}",
                T::name()
            ))
            .into()
        })
    }

    /// Get a mutable reference to a value in the spec
    pub fn get_mut<'a, T: FromPrimitiveValue>(
        &'a mut self,
        value_name: &str,
    ) -> Result<&'a mut T>
    {
        let value: &'a mut Value =
            self.values.get_mut(value_name).ok_or_else(|| {
                ErrorKind::SpecError(format!("Missing value: {}", value_name))
            })?;
        T::from_value_mut(value).ok_or_else(|| {
            ErrorKind::SpecError(format!(
                "Incorrect type, expected: {}",
                T::name()
            ))
            .into()
        })
    }

    /// Get a value from the spec, and remove it
    pub fn consume<T: FromValue>(
        &mut self,
        value_name: &str,
        consts: &Consts,
    ) -> Result<T>
    {
        let value: Value = self.values.remove(value_name).ok_or_else(|| {
            ErrorKind::SpecError(format!("Missing value: {}", value_name))
        })?;
        T::from_value(value, consts).chain_err(|| {
            format!("Failed to consume {} as type {}", value_name, T::name())
        })
    }

    /// Get a value from the spec, and remove it. If it doesn't exist, return
    /// `default`
    pub fn consume_with_default<T: FromValue>(
        &mut self,
        value_name: &str,
        default: T,
        consts: &Consts,
    ) -> Result<T>
    {
        match self.values.remove(value_name) {
            Some(value) => T::from_value(value, consts),
            None => Ok(default),
        }
    }

    /// Get a value from the spec, and remove it. If it doesn't exist, return
    /// `None`
    pub fn consume_optional<T: FromValue>(
        &mut self,
        value_name: &str,
        consts: &Consts,
    ) -> Result<Option<T>>
    {
        match self.values.remove(value_name) {
            Some(value) => T::from_value(value, consts).map(Some),
            None => Ok(None),
        }
    }

    /// Consume a list of elements as a type
    pub fn consume_list<T: FromValue>(
        &mut self,
        list_name: &str,
        consts: &Consts,
    ) -> Result<Vec<T>>
    {
        let value_list: Vec<Value> = if let Some(value_list) =
            self.consume_optional(list_name, consts)?
        {
            value_list
        } else {
            // If the field doesn't exist, return an empty list
            // TODO: What about showing that no list was given?
            return Ok(vec![]);
        };

        value_list
            .into_iter()
            .map(|v| v.into_type::<T>(consts))
            .collect::<Result<Vec<_>>>()
    }

    /// Add a field to the spec, returning the modified spec
    pub fn with<T: FromPrimitiveValue>(
        mut self,
        value_name: String,
        value: T,
    ) -> Spec
    {
        self.values.insert(value_name, value.into_value());
        self
    }

    /// Add a field to the spec
    pub fn put<T: FromPrimitiveValue>(&mut self, value_name: String, value: T) {
        self.values.insert(value_name, value.into_value());
    }

    /// Check that all values in the spec have been used
    pub fn ensure_all_used(&self) -> Result<()> {
        if self.values.is_empty() {
            Ok(())
        } else {
            Err(ErrorKind::SpecError(format!(
                "Extra values in spec: {:?}",
                self.values.keys().cloned().collect::<Vec<_>>(),
            ))
            .into())
        }
    }

    /// Get the names of the values in the spec
    pub fn value_names(&self) -> Vec<String> {
        self.values.keys().cloned().collect()
    }
}

impl Value {
    /// Try return the value as a type
    pub fn into_type<T: FromValue>(self, consts: &Consts) -> Result<T> {
        T::from_value(self, consts)
    }
}
