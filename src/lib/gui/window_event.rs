use sfml::window::{Event, Key};

use error::*;
use std::str::FromStr;

/// Events that happen on the window.
#[derive(PartialEq, Eq, Debug)]
pub enum WindowEvent {
    /// Key pressed.
    KeyPressed(WindowKey),
    /// Key released.
    KeyReleased(WindowKey),
}

impl WindowEvent {
    /// Constructs from an SFML event.
    pub fn from_event(event: &Event) -> Option<Self> {
        match event {
            Event::KeyPressed { code, .. } => Some(WindowEvent::KeyPressed(code.clone().into())),
            Event::KeyReleased { code, .. } => Some(WindowEvent::KeyReleased(code.clone().into())),
            _ => None,
        }
    }
}

/// A key pressed in the window.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct WindowKey {
    sfml_key: sfml::window::Key,
}

impl From<sfml::window::Key> for WindowKey {
    fn from(sfml_key: Key) -> Self {
        WindowKey { sfml_key }
    }
}

impl FromStr for WindowKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let sfml_key = match s {
            "a" => sfml::window::Key::A,
            "b" => sfml::window::Key::B,
            "c" => sfml::window::Key::C,
            "d" => sfml::window::Key::D,
            "e" => sfml::window::Key::E,
            "f" => sfml::window::Key::F,
            "g" => sfml::window::Key::G,
            "h" => sfml::window::Key::H,
            "i" => sfml::window::Key::I,
            "j" => sfml::window::Key::J,
            "k" => sfml::window::Key::K,
            "l" => sfml::window::Key::L,
            "m" => sfml::window::Key::M,
            "n" => sfml::window::Key::N,
            "o" => sfml::window::Key::O,
            "p" => sfml::window::Key::P,
            "q" => sfml::window::Key::Q,
            "r" => sfml::window::Key::R,
            "s" => sfml::window::Key::S,
            "t" => sfml::window::Key::T,
            "u" => sfml::window::Key::U,
            "v" => sfml::window::Key::V,
            "w" => sfml::window::Key::W,
            "x" => sfml::window::Key::X,
            "y" => sfml::window::Key::Y,
            "z" => sfml::window::Key::Z,
            key => bail!(ErrorKind::SpecError(format!("Unrecognized key {}", key))),
        };
        Ok(sfml_key.into())
    }
}
