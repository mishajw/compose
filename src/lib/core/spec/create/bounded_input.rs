use core::input;
use core::spec::create::create_with_type;
use core::spec::Spec;
use core::Consts;
use error::*;
use inputs;

/// Create an input from the spec. Every creatable input has to be added to
/// this function
pub fn create_bounded_input(
    spec: &mut Spec,
    consts: &Consts,
) -> Result<Box<input::Bounded>>
{
    let name: String = spec.consume("name")?;
    create_with_type::<inputs::Function, _>(&name, spec, consts)
        .or_else(|| {
            create_with_type::<inputs::BoolToBounded, _>(&name, spec, consts)
        })
        .or_else(|| {
            create_with_type::<inputs::SmoothBool, _>(&name, spec, consts)
        })
        .unwrap_or_else(|| Err(ErrorKind::SpecUnknownName(name.clone()).into()))
        .chain_err(|| format!("Failed to create bounded input {}", name))
}
