use core::spec::Spec;
use core::spec::SpecField;
use core::spec::SpecFieldDescription;
use core::spec::SpecType;
use core::tree::Tree;
use core::Consts;
use core::Input;
use core::State;
use core::Time;
use error::*;
use inputs::Function;

field_decl!(INPUT, Box<Input>, "Input to smooth");
field_decl!(SMOOTH_IN, Time, "How long to smooth in for");
field_decl!(SMOOTH_OUT, Time, "How long to smooth out for");
field_decl!(SMOOTH_FN, Box<Input>, "Smoothing function", |_| Box::new(
    Function::with_mod(Box::new(|x| x), Time::Seconds(1.0), false)
)
    as Box<Input>);

/// Smooth a bool transition
pub struct SmoothBool {
    bool_input: Box<Input>,
    smooth_fn: Box<Input>,
    smooth_in_duration: Time,
    smooth_out_duration: Time,
    activation: f64,
}

impl SmoothBool {
    #[allow(missing_docs)]
    pub fn bounded(
        bool_input: Box<Input>,
        smooth_fn: Box<Input>,
        smooth_in_duration: Time,
        smooth_out_duration: Time,
    ) -> SmoothBool
    {
        // TODO: Enfore smooth_fn limit [0, 1]
        SmoothBool {
            bool_input,
            smooth_fn,
            smooth_in_duration,
            smooth_out_duration,
            activation: 0.0,
        }
    }
}

impl Input for SmoothBool {
    fn get(&mut self, state: &State) -> f64 {
        let input = self.bool_input.get_bool(state);
        if input && self.activation < 1.0 {
            self.activation +=
                1.0 / self.smooth_in_duration.to_ticks(&state.consts) as f64;
            self.activation = self.activation.min(1.0);
        } else if !input && self.activation > 0.0 {
            self.activation -=
                1.0 / self.smooth_out_duration.to_ticks(&state.consts) as f64;
            self.activation = self.activation.max(0.0);
        }
        self.smooth_fn.get(
            &state.with_tick(
                Time::Seconds(self.activation).to_ticks(&state.consts),
            ),
        )
    }
}

impl Tree for SmoothBool {
    fn to_tree(&self) -> &Tree { self as &Tree }

    fn get_children<'a>(&'a self) -> Vec<&'a Tree> {
        vec![self.bool_input.to_tree()]
    }
}

impl SpecType for SmoothBool {
    fn name() -> String { "smooth-bool".into() }

    fn field_descriptions() -> Vec<SpecFieldDescription> {
        vec![
            INPUT.to_description(),
            SMOOTH_IN.to_description(),
            SMOOTH_OUT.to_description(),
            SMOOTH_FN.to_description(),
        ]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Self> {
        let input = INPUT.get(&mut spec, consts)?;
        let smooth_in = SMOOTH_IN.get(&mut spec, consts)?;
        let smooth_out = SMOOTH_OUT.get(&mut spec, consts)?;
        let smooth_fn = SMOOTH_FN.get(&mut spec, consts)?;
        Ok(SmoothBool::bounded(input, smooth_fn, smooth_in, smooth_out))
    }
}
