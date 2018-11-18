use core::input;
use core::spec::create::create_with_type;
use core::spec::Spec;
use errors::*;
use inputs;

/// Create an input from the spec. Every creatable input has to be added to
/// this function
pub fn create_bounded_input(spec: &mut Spec) -> Result<Box<input::Bounded>> {
    let name: String = spec.consume("name")?;
    create_with_type::<inputs::Wave, _>(&name, spec)
        .or_else(|| create_with_type::<inputs::BoolToBounded, _>(&name, spec))
        .or_else(|| create_with_type::<inputs::SmoothBool, _>(&name, spec))
        .unwrap_or_else(|| Err(ErrorKind::SpecUnknownName(name.clone()).into()))
        .chain_err(|| format!("Failed to create bounded input {}", name))
}
