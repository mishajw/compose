extern crate composer;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;

use composer::error::*;

const DEFAULT_CONFIG_PATH: &str = "./composer.config";

quick_main!(run);

fn run() -> Result<()> {
    // Initialize command line arguments
    let matches = clap_app!(composer =>
        (@arg spec_path: -s --spec +takes_value +required
         "Specification of the composition")
        (@arg config_path: -c --config +takes_value
         "Configuration file")
    )
    .get_matches();
    let spec_path = matches.value_of("spec_path").unwrap();
    let config_path = matches
        .value_of("config_path")
        .unwrap_or(DEFAULT_CONFIG_PATH);

    // Initialize logging
    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "composer=debug");
    env_logger::Builder::from_env(env).init();

    composer::core::composer::compose_from_file(
        spec_path.into(),
        config_path.into(),
    )?;
    Ok(())
}
