use core::spec::create::create_with_type;
use core::spec::Spec;
use core::Consts;
use core::Player;
use error::*;
use players;

/// Create a player from the spec. Every creatable player has to be added to
/// this function
pub fn create_player(spec: &mut Spec, consts: &Consts) -> Result<Box<Player>> {
    let name: String = spec.consume("name")?;
    create_with_type::<players::Wave, _>(&name, spec, consts)
        .or_else(|| create_with_type::<players::Volume, _>(&name, spec, consts))
        .or_else(|| {
            create_with_type::<players::Combiner, _>(&name, spec, consts)
        })
        .or_else(|| create_with_type::<players::Toggle, _>(&name, spec, consts))
        .or_else(|| {
            create_with_type::<players::Keyboard, _>(&name, spec, consts)
        })
        .or_else(|| create_with_type::<players::Sample, _>(&name, spec, consts))
        .or_else(|| {
            create_with_type::<players::WaveDrawer, _>(&name, spec, consts)
        })
        .or_else(|| create_with_type::<players::Speed, _>(&name, spec, consts))
        .or_else(|| create_with_type::<players::Empty, _>(&name, spec, consts))
        .unwrap_or_else(|| Err(ErrorKind::SpecUnknownName(name.clone()).into()))
        .chain_err(|| format!("Failed to create player {}", name))
}
