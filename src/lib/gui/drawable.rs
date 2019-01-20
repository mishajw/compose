use error::*;

use sfml::graphics::RenderWindow;

/// Implementors can be drawn to a GUI
pub trait Drawable {
    /// Draw onto a window inside the given bounds
    fn draw(
        &self,
        window: &mut RenderWindow,
        width: u32,
        height: u32,
        offset_x: u32,
        offset_y: u32,
    ) -> Result<()>;
}
