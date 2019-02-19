use core::spec::read::yaml_string_to_spec;
use core::spec::Spec;
use error::*;

use std::process::Command;

/// Execute a python file and return the spec
pub fn python_string_to_spec(python_str: String) -> Result<Spec> {
    let output = Command::new("python")
        .args(&["-c", &python_str])
        .output()
        .chain_err(|| "Failed to execute python file for spec")?;
    let stdout = String::from_utf8(output.stdout)
        .chain_err(|| "Failed to parse python stdout as utf8")?;
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr)
            .chain_err(|| "Failed to parse python stderr as utf8")?;
        bail!(ErrorKind::ExecutionError(format!(
            "Python script returned bad error code.\nstdout:\n{}\nstderr:\n{}",
            stdout, stderr
        )));
    }
    yaml_string_to_spec(stdout)
}
