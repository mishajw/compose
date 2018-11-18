use core::spec::create::create_with_type;
use core::spec::Spec;
use core::Player;
use errors::*;
use players;

/// Create a player from the spec. Every creatable player has to be added to
/// this function
pub fn create_player(spec: &mut Spec) -> Result<Box<Player>> {
    let name: String = spec.consume("name")?;
    create_with_type::<players::Wave, _>(&name, spec)
        .or_else(|| create_with_type::<players::Volume, _>(&name, spec))
        .or_else(|| create_with_type::<players::Combiner, _>(&name, spec))
        .or_else(|| create_with_type::<players::Toggle, _>(&name, spec))
        .or_else(|| create_with_type::<players::Keyboard, _>(&name, spec))
        .unwrap_or_else(|| Err(ErrorKind::SpecUnknownName(name.clone()).into()))
        .chain_err(|| format!("Failed to create player {}", name))
}
