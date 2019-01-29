//! Error types used across the project

error_chain! {
    errors {
        /// Error when parsing the spec
        SpecError(message: String) {
            display("Error when parsing spec: {}", message)
        }
    }
}
