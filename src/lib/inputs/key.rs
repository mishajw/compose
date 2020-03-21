use core::spec::SpecField;
use core::spec::{Spec, SpecFieldDescription, SpecType};
use core::tree::Tree;
use core::{Consts, Input, State};
use error::*;
use gui::{WindowEvent, WindowKey, WindowListener};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};

field_decl!(KEY, String, "Key to check for");

/// Input triggered by a key press.
pub struct Key {
    key: WindowKey,
    pressed: AtomicBool,
}

impl Input for Key {
    fn get(&mut self, _state: &State) -> f64 {
        if self.pressed.load(Ordering::Relaxed) {
            1.0
        } else {
            0.0
        }
    }
}

impl WindowListener for Key {
    fn receive(&self, event: &WindowEvent) {
        if event == &WindowEvent::KeyPressed(self.key) {
            self.pressed.store(true, Ordering::Relaxed)
        } else if event == &WindowEvent::KeyReleased(self.key) {
            self.pressed.store(false, Ordering::Relaxed)
        }
    }
}

impl Tree for Key {
    fn to_tree(&self) -> &dyn Tree {
        self
    }

    fn get_listeners(&self) -> Vec<&dyn WindowListener> {
        vec![self]
    }
}

impl SpecType for Key {
    fn name() -> String {
        "key".to_string()
    }

    fn field_descriptions() -> Vec<SpecFieldDescription> {
        vec![KEY.to_description()]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Key> {
        let key = KEY.get(&mut spec, consts)?;
        Ok(Key {
            key: WindowKey::from_str(&key)?,
            pressed: AtomicBool::new(false),
        })
    }
}
