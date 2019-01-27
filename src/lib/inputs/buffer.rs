use core::input;
use core::tree::Tree;
use core::State;

/// Play from a list of playables
pub struct Buffer {
    buffer: Vec<f32>,
    lower_bound: f32,
    upper_bound: f32,
}

impl Buffer {
    #[allow(missing_docs)]
    pub fn bounded(buffer: Vec<f32>) -> Box<input::Bounded> {
        assert!(!buffer.is_empty());
        Box::new(Buffer {
            lower_bound: *buffer
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap(),
            upper_bound: *buffer
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap(),
            buffer,
        })
    }
}

impl input::Bounded for Buffer {
    fn get(&mut self, state: &State) -> f32 {
        self.buffer[state.tick % self.buffer.len()]
    }

    fn get_bounds(&self) -> (f32, f32) { (self.lower_bound, self.upper_bound) }
}

impl Tree for Buffer {
    fn to_tree(&self) -> &Tree { self as &Tree }
}
