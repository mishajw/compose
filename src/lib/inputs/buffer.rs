use core::tree::Tree;
use core::Input;
use core::State;

/// Play from a list of playables
pub struct Buffer {
    buffer: Vec<f64>,
}

impl Buffer {
    #[allow(missing_docs)]
    pub fn new(buffer: Vec<f64>) -> Box<dyn Input> {
        assert!(!buffer.is_empty());
        let min = *buffer
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let max = *buffer
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        Box::new(Buffer {
            buffer: buffer.iter().map(|f| (f - min) / (max - min)).collect(),
        })
    }
}

impl Input for Buffer {
    fn get(&mut self, state: &State) -> f64 {
        self.buffer[state.tick() % self.buffer.len()]
    }
}

impl Tree for Buffer {
    fn to_tree(&self) -> &dyn Tree {
        self as &dyn Tree
    }
}
