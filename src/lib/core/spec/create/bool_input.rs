use core::input;
use core::spec::create::create_with_type;
use core::spec::Spec;
use errors::*;
use inputs;

/// Create an bool input from the spec. Every creatable bool input has to be
/// added to this function
pub fn create_bool_input(spec: &mut Spec) -> Result<Box<input::Bool>> {
    let name: String = spec.consume("name")?;
    create_with_type::<inputs::BoundedToBool, _>(&name, spec)
        .or_else(|| create_with_type::<inputs::Timeline, _>(&name, spec))
        .unwrap_or_else(|| Err(ErrorKind::SpecUnknownName(name.clone()).into()))
        .chain_err(|| format!("Failed to create bool input {}", name))
}
