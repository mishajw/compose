//! Error types used across the project

error_chain! {
    errors {
        /// Error in the type of a value in the `Spec`
        SpecTypeError(
            value_name: String,
            expected_type: String,
            actual_type: String
        ) {
            display(
                "Bad type in spec for value {}: expected {}, actual {}",
                value_name, expected_type, actual_type)
        }
        /// Expected value was not found in the `Spec`
        SpecMissingError(value_name: String) {
            display("Value missing from spec: {}", value_name)
        }
        /// Extra unused values were found in the `Spec`
        SpecExtraValuesError(value_names: Vec<String>) {
            display("Extra values found in spec: {:?}", value_names)
        }
        /// An unknown name was found in the `Spec`
        SpecUnknownName(name: String) {
            display("Unknown name in spec: {}", name)
        }
    }
}
