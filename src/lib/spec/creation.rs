//! Tools for creating components from a [`Spec`](../struct.Spec.html)
//!
//! TODO: Clean up the creation functions

use core::input;
use core::Player;
use errors::*;
use inputs;
use players;
use spec::Spec;

/// Implementors can be created from a spec
pub trait FromSpec {
    /// The name of the value to be created, used to find the type of the
    /// definition
    fn name() -> &'static str;
    /// Create the value from a spec
    fn from_spec(spec: &mut Spec) -> Result<Box<Self>>;
}

#[allow(unused)]
fn create_with_type<T: 'static + FromSpec>(
    name: &str,
    spec: &mut Spec,
) -> Result<Option<Box<T>>>
{
    if name == T::name() {
        Ok(Some(T::from_spec(spec)?))
    } else {
        Ok(None)
    }
}

/// Create a player from the spec. Every creatable player has to be added to
/// this function
pub fn create_player(spec: &mut Spec) -> Result<Box<Player>> {
    fn to_player<T: 'static + Player>(player: Box<T>) -> Box<Player> { player }
    #[allow(unused)]
    let name = spec.use_str("name")?;
    let player = create_with_type::<players::Wave>(&name, spec)?.map(to_player);
    player.ok_or_else(|| ErrorKind::SpecUnknownName(name).into())
}

/// Create an input from the spec. Every creatable input has to be added to
/// this function
pub fn create_continuous_input(
    spec: &mut Spec,
) -> Result<Box<input::Continuous>> {
    fn to_input<T: 'static + input::Continuous>(
        player: Box<T>,
    ) -> Box<input::Continuous> {
        player
    }
    #[allow(unused)]
    let name = spec.use_str("name")?;
    let input: Option<Box<input::Continuous>> =
        create_with_type::<inputs::Wave>(&name, spec)?.map(to_input);
    input.ok_or_else(|| ErrorKind::SpecUnknownName(name).into())
}
