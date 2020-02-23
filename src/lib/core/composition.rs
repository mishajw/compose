use core::spec;
use core::spec::FromValue;
use core::spec::{Spec, Value};
use core::Consts;
use core::Output;
use core::Player;
use error::*;

use std::sync::Mutex;

/// A composition to be played
pub struct Composition {
    /// Tree of all players in the composition
    pub root_player: Mutex<Box<dyn Player>>,
    /// Outputs to play the composition to
    pub outputs: Mutex<Vec<Box<dyn Output>>>,
    /// Constants shared across the composition
    pub consts: Consts,
}

impl FromValue for Composition {
    fn name() -> String {
        "composition".into()
    }

    fn from_value(value: Value, consts: &Consts) -> Result<Composition> {
        let mut spec: Spec = value.into_type(consts)?;

        // Initialize consts
        let consts = spec
            .consume_with_default("consts", Consts::default()?, &Consts::default()?)
            .chain_err(|| "Failed to create consts")?;

        // Initialize players
        let player_spec_with_macros = spec.consume("players", &consts)?;
        debug!("Player spec: {:#?}", player_spec_with_macros);
        // TODO: Get rid of Value::Spec
        let player_spec = Value::Spec(spec::resolve_root_macros(player_spec_with_macros, &consts)?);
        debug!("Player spec resolved: {:#?}", player_spec);
        let root_player = player_spec.into_type(&consts)?;

        // Initialize outputs
        let outputs: Vec<Box<dyn Output>> = spec.consume_list("outputs", &consts)?;
        spec.ensure_all_used()?;

        Ok(Composition {
            root_player: Mutex::new(root_player),
            outputs: Mutex::new(outputs),
            consts,
        })
    }
}
