use core::spec::Spec;
use core::spec::SpecField;
use core::spec::SpecFieldDescription;
use core::spec::SpecType;
use core::tree::Tree;
use core::Consts;
use core::Playable;
use core::Player;
use core::State;
use error::*;

use num::rational::Ratio;
use num::traits::ToPrimitive;

field_decl!(CHILD, Box<dyn Player>, "Child to adjust the speed of");
field_decl!(SPEED, f64, "Speed adjustment factor");

/// Adjust the speed of a child player
pub struct Speed {
    child: Box<dyn Player>,
    scale_numerator: usize,
    scale_denominator: usize,
}

impl Speed {
    #[allow(missing_docs)]
    pub fn player(child: Box<dyn Player>, scale: f64) -> Result<Speed> {
        Ok(Speed::new(child, scale)?)
    }

    fn new(child: Box<dyn Player>, scale: f64) -> Result<Speed> {
        let ratio = match Ratio::from_float(scale) {
            Some(ratio) => ratio,
            None => bail!(ErrorKind::SpecError(format!(
                "Failed to convert scale {} into ratio",
                scale
            ))),
        };

        Ok(Speed {
            child,
            scale_numerator: ratio
                .numer()
                .to_usize()
                .chain_err(|| "Failed to convert scale numerator into i32")?,
            scale_denominator: ratio
                .denom()
                .to_usize()
                .chain_err(|| "Failed to convert scale denominator into i32")?,
        })
    }

    fn scale(&self, value: usize) -> usize {
        ((value as u128 * self.scale_numerator as u128) / self.scale_denominator as u128) as usize
    }
}

impl Player for Speed {
    fn play(&mut self, state: &State) -> Playable {
        let scaled_tick = self.scale(state.milli_tick);
        self.child.play(&state.with_milli_tick(scaled_tick))
    }
}

impl Tree for Speed {
    fn to_tree(&self) -> &dyn Tree {
        self as &dyn Tree
    }

    fn get_children(&self) -> Vec<&dyn Tree> {
        vec![self.child.to_tree()]
    }
}

impl SpecType for Speed {
    fn name() -> String {
        "speed".into()
    }

    fn field_descriptions() -> Vec<SpecFieldDescription> {
        vec![CHILD.to_description(), SPEED.to_description()]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Self> {
        let child = CHILD.get(&mut spec, consts)?;
        let speed = SPEED.get(&mut spec, consts)?;
        spec.ensure_all_used()?;
        Speed::player(child, speed)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use players::Empty;
    use players::Linear;

    use std::sync::Arc;

    #[test]
    fn test_scale_accuracy() {
        test_ranges(1.0);
        test_ranges(1.5);
        test_ranges(2.0);
        test_ranges(0.01);
        test_ranges(0.001);
        test_ranges(1.001);
        test_ranges(1.01);
        test_ranges(1.03);
        test_ranges(100.0001);
    }

    fn test_ranges(scale: f64) {
        println!("Testing scale {}", scale);
        let speed = Speed::new(Box::new(Empty::player()), scale).unwrap();
        test_range(&speed, 0, 100, 1);
        test_range(&speed, 1000000, 2000000, 100000);
        test_range(&speed, 10000000, 20000000, 10000);
        test_range(&speed, 1000000000, 2000000000, 100000);
    }

    fn test_range(speed: &Speed, start: usize, end: usize, incr: usize) {
        println!("Testing range {}-{} with step {}", start, end, incr);
        let values: Vec<usize> = (start..end).step_by(incr).map(|v| speed.scale(v)).collect();
        let value_diffs: Vec<i64> = values
            .iter()
            .zip(values.iter().skip(1))
            .map(|(v1, v2)| *v2 as i64 - *v1 as i64)
            .collect();
        let first_diff = value_diffs[0];
        for diff in value_diffs {
            assert!((diff - first_diff).abs() <= 1);
        }
    }

    #[test]
    fn test_double() {
        let scale = 1.1;
        let mut speed0 = Speed::new(Box::new(Linear::player(100)), scale * scale).unwrap();
        let mut speed1 = Speed::new(
            Box::new(Speed::new(Box::new(Linear::player(100)), scale).unwrap()),
            scale,
        )
        .unwrap();

        let mut state = State::initial(Arc::new(Consts::default().unwrap()));
        loop {
            let played0 = speed0.play(&state).get_value();
            let played1 = speed1.play(&state).get_value();
            let diff = dbg!((played0 - played1).abs());
            assert!(diff <= 1);
            state.increment();
            if state.tick() > 100000 {
                break;
            }
        }
    }
}
