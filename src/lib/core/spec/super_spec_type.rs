use core::spec::FromValue;
use core::spec::SpecTypeDescription;

/// Implementors will have a list of sub types used to instantiate them
pub trait SuperSpecType: FromValue {
    /// Get the descriptions of all sub types
    fn sub_type_descriptions() -> Vec<SpecTypeDescription>;
}

/// Macro for implementing `FromValue` that selects which sub type to create
/// based of the `name` parameter
macro_rules! impl_super_from_value {
    ($super_type:ty, $super_name:expr, $($sub_types:ty),*) => {
        use core::Consts;
        use core::spec::SpecType;
        use core::spec::SpecTypeDescription;
        use core::spec::SuperSpecType;
        use core::spec::{Value, FromValue, Spec};
        use error::*;

        use std::collections::HashMap;

        type GetFn =
            Box<Fn(Value, &Consts) -> Result<Box<$super_type>> + Send + Sync>;

        lazy_static! {
            static ref MAP: HashMap<String, GetFn> = {
                let mut map = HashMap::new();
                $(
                    let create_fn: GetFn = Box::new(|v: Value, c: &Consts| {
                        let created = <$sub_types>::from_value(v, c);
                        created.map(|v| -> Box<$super_type> { Box::new(v) })
                    });
                    map.insert(
                        <$sub_types as SpecType<_>>::name().to_string(),
                        create_fn
                    );
                )*
                map
            };
        }

        impl FromValue for Box<$super_type> {
            fn name() -> &'static str { $super_name }

            fn from_value(value: Value, consts: &Consts) -> Result<Self> {
                let mut spec: Spec = value.into_type(consts)?;
                let name: String = spec.consume("name", consts)
                    .chain_err(|| format!(
                        "Couldn't find name for {}", Self::name()
                    ))?;
                match MAP.get(&name) {
                    Some(create_fn) => {
                        create_fn(Value::Spec(spec), consts)
                            .chain_err(|| format!("Failed to create {}", name))
                    },
                    None => bail!(ErrorKind::SpecError(format!(
                        "Unrecognized name: {}", name
                    )))
                }
            }
        }

        impl SuperSpecType for Box<$super_type> {
            fn sub_type_descriptions() -> Vec<SpecTypeDescription> {
                vec![$(
                    <$sub_types as SpecType<_>>::to_description(),
                )*]
            }
        }
    };
}
