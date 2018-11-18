//! Create compositions

use errors::*;
use core::CompositionConsts;
use core::CompositionState;
use core::Output;
use core::Player;
use core::spec;
use core::spec::create::FromSpec;
use std::path::Path;

/// Start a composition from a file
pub fn compose_from_file(path: &Path) -> Result<()> {
    info!("Reading yaml spec from {:?}", path);
    let spec = spec::yaml::read(path)?;
    compose_from_spec(spec)
}

/// Start a composition from a spec
pub fn compose_from_spec(mut spec: spec::Spec) -> Result<()> {
    // Initialize players
    let player_spec_with_macros = spec.consume("players")?;
    debug!("Player spec: {:#?}", player_spec_with_macros);
    let mut player_spec =
        spec::create::resolve_macros(player_spec_with_macros)?;
    debug!("Player spec resolved: {:#?}", player_spec);
    let output_specs = spec.consume("outputs")?;
    let mut player = spec::create::create_player(&mut player_spec)?;

    // Initialize outputs
    let outputs = spec::create::create_outputs(output_specs)?;

    // Initialize consts
    let consts = CompositionConsts::from_spec(spec.consume_with_default(
        "consts",
        spec::Value::Spec(spec::Spec::empty()),
    )?)?;

    info!("Composing");
    compose(player.as_mut(), outputs, consts);
}

/// Create a composition from the `root_player` into the `output`s
pub fn compose(
    root_player: &mut Player,
    mut outputs: Vec<Box<Output>>,
    consts: CompositionConsts,
) -> !
{
    let mut state = CompositionState::initial(consts);
    loop {
        let played = root_player.play(&state);
        for output in outputs.iter_mut() {
            output.write(played);
        }
        state.increment();
    }
}
