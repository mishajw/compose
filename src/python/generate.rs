extern crate composer;

use composer::core::input::Bool;
use composer::core::input::Bounded;
use composer::core::spec::SpecFieldDescription;
use composer::core::spec::SpecTypeDescription;
use composer::core::spec::SuperSpecType;
use composer::core::Output;
use composer::core::Player;

use std::fs;

const OUTPUT_PATH: &str = "pycomposer/gen/__init__.py";
const PREAMBLE: &str = r"
from typing import Optional, Iterable
from pycomposer import ToSpec, Time
";

fn main() {
    let python_output = create_all();
    fs::write(OUTPUT_PATH, python_output).expect("Failed to write python file");
}

fn create_all() -> String {
    PREAMBLE.to_string()
        + &create_super_type::<Box<Bool>>()
        + &create_super_type::<Box<Bounded>>()
        + &create_super_type::<Box<Player>>()
        + &create_super_type::<Box<Output>>()
}

fn create_super_type<T: SuperSpecType>() -> String {
    let mut result = String::new();
    result += &super_type(&T::name());
    for description in <T>::sub_type_descriptions() {
        result += &sub_type(description, &T::name());
        result += "\n"
    }
    result
}

fn super_type(name: &str) -> String {
    format!(
        r"
class {}(ToSpec):
    def __init__(self, name: str, fields: dict):
        super().__init__(name, fields)
",
        class_name(name),
    )
}

fn sub_type(description: SpecTypeDescription, type_name: &str) -> String {
    if description.field_descriptions.is_empty() {
        return format!(
            r"
class {}({}):
    def __init__(self):
        super().__init__('{}', dict())",
            class_name(&description.name),
            class_name(type_name),
            description.name,
        );
    }

    let field_parameters = {
        let mut sorted_fields = description.field_descriptions.clone();
        sorted_fields.sort_by_key(|field| field.has_default);
        sorted_fields
            .iter()
            .map(field_parameter)
            .collect::<Vec<_>>()
            .join(", ")
    };
    let field_dict_entries = description
        .field_descriptions
        .iter()
        .map(field_dict_entry)
        .collect::<Vec<_>>()
        .join(", ");
    let field_initializers = description
        .field_descriptions
        .iter()
        .map(field_initializer)
        .collect::<Vec<_>>()
        .join("; ");

    format!(
        r"
class {}({}):
    def __init__(self, {}):
        {}
        super().__init__('{}', dict({}))
",
        class_name(&description.name),
        class_name(type_name),
        field_parameters,
        field_initializers,
        description.name,
        field_dict_entries,
    )
}

fn field_parameter(description: &SpecFieldDescription) -> String {
    if !description.has_default {
        format!(
            "{}: {}",
            variable_name(&description.name),
            type_name(&description.type_name)
        )
    } else {
        format!(
            "{}: Optional[{}] = None",
            variable_name(&description.name),
            class_name(&description.type_name)
        )
    }
}

fn field_initializer(description: &SpecFieldDescription) -> String {
    let variable_name = variable_name(&description.name);
    if description.type_name.ends_with("[]") {
        format!("self.{} = list({})", variable_name, variable_name)
    } else {
        format!("self.{} = {}", variable_name, variable_name)
    }
}

fn field_dict_entry(description: &SpecFieldDescription) -> String {
    let variable_name = variable_name(&description.name);
    format!("{}=self.{}", variable_name, variable_name)
}

fn type_name(s: &str) -> String {
    if s.ends_with("[]") {
        let mut s_truncated = s.to_string();
        s_truncated.truncate(s.len() - 2);
        format!("Iterable['{}']", class_name(&s_truncated))
    } else {
        format!("'{}'", class_name(s))
    }
}

fn class_name(s: &str) -> String {
    match s {
        "i32" => "int".to_string(),
        "f64" => "float".to_string(),
        "String" => "str".to_string(),
        "bool" => "bool".to_string(),
        _ => generated_class_name(s),
    }
}

fn generated_class_name(s: &str) -> String {
    let mut result = String::new();
    let mut seen_dash = true;
    for character in s.chars() {
        if character == '-' {
            seen_dash = true;
            continue;
        }
        if seen_dash {
            result += &character.to_uppercase().to_string();
        } else {
            result.push(character);
        }
        seen_dash = false;
    }
    result
}

fn variable_name(s: &str) -> String { s.to_string().replace("-", "_") }
