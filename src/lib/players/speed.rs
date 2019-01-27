use core::spec::create::create_player;
use core::spec::create::FromSpec;
use core::spec::Spec;
use core::spec::Value;
use core::tree::Tree;
use core::Consts;
use core::Playable;
use core::Player;
use core::State;
use error::*;

use num::rational::Ratio;
use num::traits::ToPrimitive;

/// Adjust the speed of a child player
pub struct Speed {
    child: Box<Player>,
    scale_numerator: usize,
    scale_denominator: usize,
}

impl Speed {
    #[allow(missing_docs)]
    pub fn player(child: Box<Player>, scale: f64) -> Result<Box<Player>> {
        Ok(Box::new(Speed::new(child, scale)?))
    }

    fn new(child: Box<Player>, scale: f64) -> Result<Speed> {
        let ratio = match Ratio::from_float(scale) {
            Some(ratio) => ratio,
            None => bail!(ErrorKind::BadInput(format!(
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
        ((value as u128 * self.scale_numerator as u128)
            / self.scale_denominator as u128) as usize
    }
}

impl Player for Speed {
    fn play(&mut self, state: &State) -> Playable {
        let scaled_tick = self.scale(state.tick);
        self.child.play(&state.with_tick(scaled_tick))
    }
}

impl Tree for Speed {
    fn to_tree(&self) -> &Tree { self as &Tree }

    fn get_children<'a>(&'a self) -> Vec<&'a Tree> {
        vec![self.child.to_tree()]
    }
}

impl FromSpec<Box<Player>> for Speed {
    fn name() -> &'static str { "speed" }

    fn from_spec(value: Value, consts: &Consts) -> Result<Box<Player>> {
        let mut spec: Spec = value.into_type()?;
        let child = create_player(&mut spec.consume("child")?, consts)?;
        let speed: f64 = spec.consume("speed")?;
        spec.ensure_all_used()?;
        Speed::player(child, f64::from(speed))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use players::Empty;

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
        let speed = Speed::new(Empty::player(), scale).unwrap();
        test_range(&speed, 0, 100, 1);
        test_range(&speed, 1000000, 2000000, 100000);
        test_range(&speed, 10000000, 20000000, 10000);
        test_range(&speed, 1000000000, 2000000000, 100000);
    }

    fn test_range(speed: &Speed, start: usize, end: usize, incr: usize) {
        println!("Testing range {}-{} with step {}", start, end, incr);
        let values: Vec<usize> =
            (start..end).step_by(incr).map(|v| speed.scale(v)).collect();
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
}
