use error::*;

use sfml::graphics::{Color, RenderWindow};

/// Implementors can be drawn to a GUI
pub trait Drawable {
    /// Draw onto a window inside the given bounds
    fn draw(
        &self,
        window: &mut RenderWindow,
        color: Color,
        width: u32,
        height: u32,
        offset_x: u32,
        offset_y: u32,
    ) -> Result<()>;
}
