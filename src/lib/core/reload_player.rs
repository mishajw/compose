use core::spec;
use core::spec::read;
use core::spec::read::ReadType;
use core::spec::Value;
use core::Consts;
use core::Player;
use error::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use error_chain::ChainedError;

type WrappedPlayer = Arc<Mutex<Box<Player>>>;

/// Returns a player that will reload in fixed intervals from a spec path
pub fn get_reloading_player(
    spec_path: String,
    read_type: ReadType,
    consts: Arc<Consts>,
) -> Result<WrappedPlayer>
{
    let mut yaml_hash: u64 = 0;
    // Safe to do unwrap, as the first load will always be new
    let yaml_str = load_spec(&spec_path, &mut yaml_hash)?.unwrap();
    let player =
        Arc::new(Mutex::new(load_player(yaml_str, read_type, &consts)?));
    start_reload_thread(
        player.clone(),
        consts.clone(),
        spec_path,
        read_type,
        yaml_hash,
    );
    Ok(player)
}

fn start_reload_thread(
    player: WrappedPlayer,
    consts: Arc<Consts>,
    spec_path: String,
    read_type: ReadType,
    mut yaml_hash: u64,
)
{
    thread::spawn(move || loop {
        if let Err(err) = reload(
            player.clone(),
            &consts,
            &spec_path,
            read_type,
            &mut yaml_hash,
        ) {
            info!("Error when reloading spec: {}", err.display_chain());
        }

        let sleep_time = { consts.reload_time.to_duration(&consts) };
        thread::sleep(sleep_time);
    });
}

fn reload(
    player: WrappedPlayer,
    consts: &Consts,
    spec_path: &str,
    read_type: ReadType,
    yaml_hash: &mut u64,
) -> Result<()>
{
    if let Some(yaml_str) = load_spec(spec_path, yaml_hash)? {
        let new_player = load_player(yaml_str, read_type, consts)?;
        *player.lock().unwrap() = new_player;
    };
    Ok(())
}

fn load_spec(
    spec_path: &str,
    current_yaml_hash: &mut u64,
) -> Result<Option<String>>
{
    let yaml_str = read::path_to_string(Path::new(spec_path))?;
    let yaml_hash = {
        let mut hasher = DefaultHasher::new();
        yaml_str.hash(&mut hasher);
        hasher.finish()
    };
    if yaml_hash == *current_yaml_hash {
        Ok(None)
    } else {
        *current_yaml_hash = yaml_hash;
        Ok(Some(yaml_str))
    }
}

fn load_player(
    yaml_str: String,
    read_type: ReadType,
    consts: &Consts,
) -> Result<Box<Player>>
{
    let spec = read::string_to_spec(yaml_str, read_type)?;
    let resolved_macros = spec::resolve_root_macros(spec, consts)?;
    // TODO
    Ok(Value::Spec(resolved_macros)
        .into_type::<Box<Player>>(consts)?
        .into())
}
