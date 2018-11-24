//! Create compositions

use core::spec;
use core::spec::create::FromSpec;
use core::CompositionConsts;
use core::CompositionState;
use core::Output;
use core::Player;
use errors::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem::swap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use error_chain::ChainedError;

/// Start a composition from a file
pub fn compose_from_file(path: String) -> Result<()> {
    info!("Reading yaml spec from {:?}", path);
    let mut spec = spec::yaml::read(&Path::new(&path))?;

    // Initialize consts
    let consts = CompositionConsts::from_spec(
        spec.consume_with_default(
            "consts",
            spec::Value::Spec(spec::Spec::empty()),
        )?,
        &CompositionConsts::default(),
    )?;

    // Initialize players
    let player_spec_with_macros = spec.consume("players")?;
    debug!("Player spec: {:#?}", player_spec_with_macros);
    let mut player_spec =
        spec::create::resolve_macros(player_spec_with_macros, &consts)?;
    debug!("Player spec resolved: {:#?}", player_spec);
    let player = spec::create::create_player(&mut player_spec, &consts)?;

    // Initialize outputs
    let output_specs = spec.consume("outputs")?;
    let outputs = spec::create::create_outputs(output_specs, &consts)?;

    spec.ensure_all_used()?;

    // Set up player reloading
    let player_replacement = Arc::new(Mutex::new(None));
    let spawn_player_replacement = player_replacement.clone();
    let spawn_path = path.clone();
    let spawn_reload_duration = consts.reload_time.to_duration(&consts);
    let spawn_consts = consts.clone();
    let mut spawn_previous_hash: u64 = 0;
    thread::spawn(move || loop {
        thread::sleep(spawn_reload_duration);
        match reload_player(
            &Path::new(&spawn_path),
            spawn_player_replacement.as_ref(),
            spawn_previous_hash,
            &spawn_consts,
        ) {
            Ok(new_hash) => spawn_previous_hash = new_hash,
            Err(err) => {
                error!("Failed to reload player: {}", err.display_chain())
            }
        }
    });

    info!("Composing");
    run_composition(player, player_replacement, outputs, consts);
}

fn reload_player(
    path: &Path,
    player: &Mutex<Option<Box<Player>>>,
    previous_hash: u64,
    consts: &CompositionConsts,
) -> Result<u64>
{
    let yaml_str = spec::yaml::get_yaml_str(path)?;
    let mut hasher = DefaultHasher::new();
    yaml_str.hash(&mut hasher);
    let yaml_hash = hasher.finish();
    if hasher.finish() == previous_hash {
        return Ok(yaml_hash);
    }
    info!("Reloading root player from {:?}", path);

    let mut spec = spec::yaml::parse(yaml_str)?;
    let mut player_spec =
        spec::create::resolve_macros(spec.consume("players")?, consts)?;
    let mut new_player =
        Some(spec::create::create_player(&mut player_spec, consts)?);
    let mut player = player.lock().unwrap();
    swap(&mut new_player, &mut *player);
    info!("Successfully set up player");
    Ok(yaml_hash)
}

/// Create a composition from the `root_player` into the `output`s
fn run_composition(
    mut player: Box<Player>,
    // TODO: christ
    player_replacement: Arc<Mutex<Option<Box<Player>>>>,
    mut outputs: Vec<Box<Output>>,
    consts: CompositionConsts,
) -> !
{
    let reload_ticks = consts.reload_time.clone().to_ticks(&consts);
    let mut state = CompositionState::initial(consts);
    loop {
        if reload_ticks != 0 && state.tick % reload_ticks == 0 {
            let mut new_player = player_replacement.lock().unwrap();
            if new_player.is_some() {
                swap(&mut player, &mut new_player.as_mut().unwrap());
                *new_player = None;
                info!("Successfully loaded player");
            }
        }

        let played = player.play(&state);
        for output in outputs.iter_mut() {
            output.write(played);
        }
        state.increment();
    }
}
