extern crate composer;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate clap;

use composer::core::spec::read;
use composer::core::spec::read::ReadType;
use composer::core::Chord;
use composer::core::Consts;
use composer::core::Note;
use composer::core::Scale;
use composer::error::*;
use composer::DEFAULT_CONFIG_PATH;

use std::path::Path;

quick_main!(run);

fn run() -> Result<()> {
    // Initialize command line arguments
    let matches = clap_app!(freq_resolver =>
        (@arg config_path: -c --config +takes_value
         "Configuration file")
        (@subcommand chord =>
            (@arg name: +takes_value +required))
        (@subcommand scale =>
            (@arg name: +takes_value +required)
            (@arg num_notes: -n --num-notes +takes_value))
    )
    .get_matches();
    let config_path = matches
        .value_of("config_path")
        .unwrap_or(DEFAULT_CONFIG_PATH);

    let consts = Consts::default()?;
    let mut spec = read::path_to_spec(Path::new(&config_path), ReadType::Yaml)?;
    let consts: Consts = spec.consume("consts", &consts)?;

    if let Some(chord_matches) = matches.subcommand_matches("chord") {
        let chord_name = chord_matches.value_of("name").unwrap();
        let chord = Chord::from_str(chord_name, &consts)?;
        print_notes(chord.into_notes());
    } else if let Some(scale_matches) = matches.subcommand_matches("scale") {
        let scale_name = scale_matches.value_of("name").unwrap();
        let scale = Scale::from_str(scale_name, &consts)?;
        let num_notes = match scale_matches.value_of("num_notes") {
            Some(num_notes) => num_notes
                .parse::<usize>()
                .chain_err(|| "Failed to parse num_notes as usize")?,
            None => scale.default_size(),
        };
        print_notes(scale.to_notes(num_notes));
    }

    Ok(())
}

fn print_notes(notes: Vec<Note>) {
    let frequency_strings = notes
        .iter()
        .map(Note::to_frequency)
        .map(|f| f.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", frequency_strings);
}
