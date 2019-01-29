//! Read `.yaml`. files into [`Spec`](../struct.Spec.html)s

use core::spec::{Spec, Value};
use error::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use yaml_rust::{Yaml, YamlLoader};

/// Read a `.yaml` file into a `Spec`
pub fn read(path: &Path) -> Result<Spec> {
    let yaml_str = get_yaml_str(path)?;
    parse(yaml_str)
}

/// Parse a `.yaml` file into a `Spec`
pub fn parse(yaml_str: String) -> Result<Spec> {
    let yaml = get_yaml(yaml_str)?;
    let value = yaml_to_value(yaml)?;
    if let Value::Spec(spec) = value {
        Ok(spec)
    } else {
        Err(ErrorKind::SpecError(
            "Top level Spec yaml must be an object".into(),
        )
        .into())
    }
}

/// Get the .yaml text
pub fn get_yaml_str(path: &Path) -> Result<String> {
    let mut file =
        File::open(path).chain_err(|| "Failed to open Spec yaml file")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .chain_err(|| "Failed to read from Spec yaml file")?;
    Ok(contents)
}

fn get_yaml(yaml_str: String) -> Result<Yaml> {
    let mut yaml_list = YamlLoader::load_from_str(&yaml_str)
        .chain_err(|| "Failed to parse Spec yaml file")?;

    if yaml_list.len() > 1 {
        return Err(ErrorKind::SpecError(format!(
            "Expected one element in the Spec yaml, found {}",
            yaml_list.len()
        ))
        .into());
    }

    if yaml_list.is_empty() {
        return Err(ErrorKind::SpecError("Empty Spec yaml file".into()).into());
    }

    Ok(yaml_list.swap_remove(0))
}

fn yaml_to_value(yaml: Yaml) -> Result<Value> {
    match yaml {
        Yaml::Hash(dict) => {
            let mut spec_values = HashMap::new();
            for (key, value) in dict {
                let value_name: Result<&str> = key.as_str().ok_or_else(|| {
                    ErrorKind::SpecError("Non-string key in Spec yaml".into())
                        .into()
                });
                let value_name: String = value_name?.into();

                spec_values.insert(
                    value_name.clone(),
                    yaml_to_value(value).chain_err(|| {
                        format!("Error parsing \"{}\" field", value_name)
                    })?,
                );
            }
            Ok(Value::Spec(Spec::new(spec_values)))
        }
        Yaml::String(string) => Ok(Value::Str(string)),
        Yaml::Integer(int) => Ok(Value::Int(int as i32)),
        Yaml::Real(float) => Ok(Value::Float(
            float
                .parse()
                .chain_err(|| "Failed to parse float in Spec yaml")?,
        )),
        Yaml::Boolean(boolean) => Ok(Value::Bool(boolean)),
        Yaml::Array(list) => {
            let values: Result<Vec<_>> =
                list.into_iter().map(yaml_to_value).collect();
            Ok(Value::List(values?))
        }
        yaml_value => Err(ErrorKind::SpecError(format!(
            "Unexpected type in Spec yaml: {:?}",
            yaml_value
        ))
        .into()),
    }
}
