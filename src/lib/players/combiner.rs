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

field_decl!(
    CHILDREN,
    Vec<Box<dyn Player>>,
    "Children players that are combined"
);

/// Sum several children `Player` output into one output
pub struct Combiner {
    children: Vec<Box<dyn Player>>,
}

impl Combiner {
    #[allow(missing_docs)]
    pub fn player(children: Vec<Box<dyn Player>>) -> Self {
        Combiner { children }
    }
}

impl Player for Combiner {
    fn play(&mut self, state: &State) -> Playable {
        self.children.iter_mut().map(|p| p.play(state)).sum()
    }
}

impl Tree for Combiner {
    fn to_tree(&self) -> &dyn Tree {
        self as &dyn Tree
    }

    fn get_children(&self) -> Vec<&dyn Tree> {
        self.children.iter().map(|c| c.to_tree()).collect()
    }
}

impl SpecType for Combiner {
    fn name() -> String {
        "combiner".into()
    }

    fn field_descriptions() -> Vec<SpecFieldDescription> {
        vec![CHILDREN.to_description()]
    }

    fn from_spec(mut spec: Spec, consts: &Consts) -> Result<Self> {
        let children = CHILDREN.get(&mut spec, consts)?;
        spec.ensure_all_used()?;
        Ok(Combiner::player(children))
    }
}
