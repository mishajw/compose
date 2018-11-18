use core::spec::create::create_with_type;
use core::spec::Spec;
use core::spec::Value;
use core::Output;
use errors::*;
use outputs;

/// Create outputs from the spec
pub fn create_outputs(values: Vec<Value>) -> Result<Vec<Box<Output>>> {
    let mut outputs = Vec::new();
    for value in values {
        if let Value::Spec(mut spec) = value {
            outputs.push(create_output(&mut spec)?);
        } else {
            return Err(ErrorKind::SpecTypeError(
                "outputs[]".into(),
                "object".into(),
            )
            .into());
        }
    }
    Ok(outputs)
}

/// Create an output from the spec. Every creatable output has to be added to
/// this function
fn create_output(spec: &mut Spec) -> Result<Box<Output>> {
    let name: String = spec.consume("name")?;
    create_with_type::<outputs::Speaker, _>(&name, spec)
        .unwrap_or_else(|| Err(ErrorKind::SpecUnknownName(name.clone()).into()))
        .chain_err(|| format!("Failed to create output {}", name))
}
