//! Create compositions

use core::ReloadingComposition;
use core::State;
use error::*;

/// Start a composition from a file
pub fn compose_from_file(path: String) -> Result<()> {
    let reloading_composition = ReloadingComposition::new(path)?;
    let mut state =
        State::initial(reloading_composition.get_composition().consts.clone());
    loop {
        let composition = reloading_composition.get_composition();
        let played = composition.root_player.lock().unwrap().play(&state);
        for output in composition.outputs.lock().unwrap().iter_mut() {
            output.write(played);
        }
        state.increment();
    }
}
