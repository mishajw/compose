use core::spec;
use core::spec::create::FromSpec;
use core::spec::Value;
use core::Composition;
use core::Consts;
use error::*;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use error_chain::ChainedError;

type WrappedComposition = Arc<Mutex<Arc<Composition>>>;

/// Wraps a composition and reloads it at fixed intervals
pub struct ReloadingComposition {
    composition: WrappedComposition,
}

impl ReloadingComposition {
    /// Start a reloading composition from a spec path
    pub fn new(spec_path: String) -> Result<ReloadingComposition> {
        let mut yaml_hash: u64 = 0;
        // Safe to do unwrap, as the first load will always be new
        let yaml_str = load_spec(&spec_path, &mut yaml_hash)?.unwrap();
        let composition =
            Arc::new(Mutex::new(Arc::new(load_composition(yaml_str)?)));
        start_reload_thread(composition.clone(), spec_path, yaml_hash);
        Ok(ReloadingComposition { composition })
    }

    /// Get most recently loaded composition
    pub fn get_composition(&self) -> Arc<Composition> {
        self.composition.lock().unwrap().clone()
    }
}

fn start_reload_thread(
    composition: WrappedComposition,
    spec_path: String,
    mut yaml_hash: u64,
)
{
    thread::spawn(move || loop {
        if let Err(err) =
            reload(composition.clone(), &spec_path, &mut yaml_hash)
        {
            info!("Error when reloading spec: {}", err.display_chain());
        }

        let sleep_time = {
            let consts = &composition.lock().unwrap().consts;
            consts.reload_time.to_duration(&consts)
        };
        thread::sleep(sleep_time);
    });
}

fn reload(
    composition: WrappedComposition,
    spec_path: &str,
    yaml_hash: &mut u64,
) -> Result<()>
{
    if let Some(yaml_str) = load_spec(spec_path, yaml_hash)? {
        let new_composition = load_composition(yaml_str)?;
        *composition.lock().unwrap() = Arc::new(new_composition);
    };
    Ok(())
}

fn load_spec(
    spec_path: &str,
    current_yaml_hash: &mut u64,
) -> Result<Option<String>>
{
    let yaml_str = spec::yaml::get_yaml_str(Path::new(spec_path))?;
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

fn load_composition(yaml_str: String) -> Result<Composition> {
    let spec = spec::yaml::parse(yaml_str)?;
    Composition::from_spec(Value::Spec(spec), &Consts::default()?)
}
