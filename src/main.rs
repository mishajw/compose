extern crate composer;
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate clap;

use composer::core::compose;
use composer::core::CompositionConsts;
use composer::errors::*;
use composer::spec;
use composer::spec::{FromSpec, Spec, Value};

use std::path::Path;

use error_chain::ChainedError;

fn main() {
    if let Err(err) = run() {
        eprintln!("Failed to run composer: {}", err.display_chain());
    }
}

fn run() -> Result<()> {
    // Initialize command line arguments
    let matches = clap_app!(composer =>
        (@arg spec_path: -s --spec +takes_value +required
         "Specification of the composition")
    )
    .get_matches();
    let spec_path = Path::new(matches.value_of("spec_path").unwrap());

    // Initialize logging
    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "composer=debug");
    env_logger::Builder::from_env(env).init();

    info!("Initializing from spec");
    let mut spec = spec::yaml::read(Path::new(spec_path))?;
    let mut player_spec = spec.consume("players")?;
    let output_specs = spec.consume("outputs")?;
    let mut player = spec::create_player(&mut player_spec)?;
    let outputs = spec::create_outputs(output_specs)?;
    let consts = CompositionConsts::from_spec(
        spec.consume_with_default("consts", Value::Spec(Spec::empty()))?,
    )?;

    info!("Composing");
    compose(player.as_mut(), outputs, consts);

    info!("Finished");
    Ok(())
}
