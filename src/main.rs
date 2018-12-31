extern crate composer;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;

use composer::error::*;

quick_main!(run);

fn run() -> Result<()> {
    // Initialize command line arguments
    let matches = clap_app!(composer =>
        (@arg spec_path: -s --spec +takes_value +required
         "Specification of the composition")
    )
    .get_matches();
    let spec_path = matches.value_of("spec_path").unwrap();

    // Initialize logging
    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "composer=debug");
    env_logger::Builder::from_env(env).init();

    composer::core::composer::compose_from_file(spec_path.into())?;
    Ok(())
}
