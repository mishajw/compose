//! Functions for reading specs from files

use core::spec::Spec;
use error::*;

use std::fs::File;
use std::io::Read;
use std::path::Path;

mod python;
mod yaml;

pub use self::python::python_string_to_spec;
pub use self::yaml::yaml_string_to_spec;

/// Methods to read and parse a spec file
#[derive(Copy, Clone)]
pub enum ReadType {
    /// Read a `.yaml` file
    Yaml,
    /// Read and execute a `.py` file
    Python,
}

/// Read a `Spec` from a path
pub fn path_to_spec(path: &Path, read_type: ReadType) -> Result<Spec> {
    string_to_spec(path_to_string(path)?, read_type)
}

/// Parse a `Spec` from a string
pub fn string_to_spec(string: String, read_type: ReadType) -> Result<Spec> {
    match read_type {
        ReadType::Yaml => yaml_string_to_spec(string),
        ReadType::Python => python_string_to_spec(string),
    }
}

/// Get string from a path
pub fn path_to_string(path: &Path) -> Result<String> {
    let mut file =
        File::open(path).chain_err(|| "Failed to open configuration file")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .chain_err(|| "Failed to read from configuration file")?;
    Ok(contents)
}
