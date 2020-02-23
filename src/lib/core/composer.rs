//! Create compositions

use core::get_reloading_player;
use core::spec::read;
use core::spec::read::ReadType;
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
    read_type: ReadType,
    config_path: String,
) -> Result<()> {
    let (consts, mut outputs) = get_from_config(config_path)?;
    let consts = Arc::new(consts);
    let reloading_player = get_reloading_player(composition_path, read_type, consts.clone())?;
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

fn get_from_config(config_path: String) -> Result<(Consts, Vec<Box<dyn Output>>)> {
    let consts = Consts::default()?;
    let mut spec = read::path_to_spec(Path::new(&config_path), ReadType::Yaml)?;
    let consts: Consts = spec.consume("consts", &consts)?;
    let outputs: Vec<Box<dyn Output>> = spec.consume("outputs", &consts)?;
    Ok((consts, outputs))
}
