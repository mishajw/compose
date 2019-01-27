use core::spec;
use core::spec::create::FromSpec;
use core::spec::{Spec, Value};
use core::Consts;
use core::Output;
use core::Player;
use error::*;

use std::sync::{Arc, Mutex};

/// A composition to be played
pub struct Composition {
    /// Tree of all players in the composition
    pub root_player: Mutex<Box<Player>>,
    /// Outputs to play the composition to
    pub outputs: Mutex<Vec<Box<Output>>>,
    /// Constants shared across the composition
    pub consts: Arc<Consts>,
}

impl FromSpec<Composition> for Composition {
    fn name() -> &'static str { "composition" }

    fn from_spec(value: Value, _consts: &Consts) -> Result<Composition> {
        let mut spec: Spec = value.into_type()?;

        // Initialize consts
        let consts = Arc::new(
            Consts::from_spec(
                spec.consume_with_default(
                    "consts",
                    spec::Value::Spec(spec::Spec::empty()),
                )?,
                &Consts::default()?,
            )
            .chain_err(|| "Failed to create consts")?,
        );

        // Initialize players
        let player_spec_with_macros = spec.consume("players")?;
        debug!("Player spec: {:#?}", player_spec_with_macros);
        let mut player_spec = spec::create::resolve_root_macros(
            player_spec_with_macros,
            &consts,
        )?;
        debug!("Player spec resolved: {:#?}", player_spec);
        let root_player =
            Mutex::new(spec::create::create_player(&mut player_spec, &consts)?);

        // Initialize outputs
        let output_specs = spec.consume("outputs")?;
        let outputs =
            Mutex::new(spec::create::create_outputs(output_specs, &consts)?);

        spec.ensure_all_used()?;

        Ok(Composition {
            root_player,
            outputs,
            consts,
        })
    }
}
