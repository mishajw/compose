//! Code for treating the composition as a tree of players and inputs

/// Implementors return children `Tree`s, and also children of specific types
pub trait Tree {
    /// Get a reference to self as a tree
    ///
    /// Need this to convert `Player` and input types to `Tree` types, see:
    /// https://stackoverflow.com/questions/28632968/why-doesnt-rust-support-trait-object-upcasting
    fn to_tree<'a>(&'a self) -> &'a Tree;

    /// Get the children of this tree
    fn get_children<'a>(&'a self) -> Vec<&'a Tree> { Vec::new() }
}
