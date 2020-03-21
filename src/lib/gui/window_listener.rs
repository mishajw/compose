use gui::WindowEvent;

/// Listens to window events.
pub trait WindowListener {
    /// Receives a window event.
    ///
    /// Modifications must be done under mutexes (&self is not mutable) as key presses are detected
    /// in a separate thread.
    fn receive(&self, event: &WindowEvent);
}
