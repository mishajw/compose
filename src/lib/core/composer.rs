//! Create compositions

use core::get_reloading_player;
use core::spec::yaml;
use core::Consts;
use core::Output;
use core::State;
use error::*;
use gui;

use std::path::Path;
use std::sync::Arc;

/// Start a composition from a file
pub fn compose_from_file(
    composition_path: String,
    config_path: String,
) -> Result<()>
{
    let (consts, mut outputs) = get_from_config(config_path)?;
    let consts = Arc::new(consts);
    let reloading_player =
        get_reloading_player(composition_path, consts.clone())?;
    gui::start(reloading_player.clone())?;
    let mut state = State::initial(consts.clone());
    loop {
        let played = reloading_player.lock().unwrap().play(&state);
        for output in outputs.iter_mut() {
            output.write(played);
        }
        state.increment();
    }
}

fn get_from_config(config_path: String) -> Result<(Consts, Vec<Box<Output>>)> {
    let consts = Consts::default()?;
    let mut spec = yaml::read(Path::new(&config_path))?;
    let consts: Consts = spec.consume("consts", &consts)?;
    let outputs: Vec<Box<Output>> = spec.consume("outputs", &consts)?;
    Ok((consts, outputs))
}
