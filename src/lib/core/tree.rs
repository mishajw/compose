//! Code for treating the composition as a tree of players and inputs.

use gui::{Drawable, WindowListener};

use std::iter::once;

/// Implementors return children `Tree`s, and also children of specific types.
///
/// The only specific type child implemented is for `Drawable`.
pub trait Tree {
    /// Gets a reference to self as a tree.
    ///
    /// Need this to convert `Player` and input types to `Tree` types, see:
    /// https://stackoverflow.com/questions/28632968/why-doesnt-rust-support-trait-object-upcasting
    fn to_tree(&self) -> &dyn Tree;

    /// Gets the children of this tree.
    fn get_children(&self) -> Vec<&dyn Tree> {
        Vec::new()
    }

    /// Gets the child drawables of this tree.
    fn get_drawables(&self) -> Vec<&dyn Drawable> {
        Vec::new()
    }

    /// Gets the child listeners of the tree.
    fn get_listeners(&self) -> Vec<&dyn WindowListener> {
        Vec::new()
    }
}

/// Returns a list of all nodes in a tree.
pub fn flatten_tree(root: &dyn Tree) -> Vec<&dyn Tree> {
    root.get_children()
        .into_iter()
        .map(flatten_tree)
        .flat_map(IntoIterator::into_iter)
        .chain(once(root))
        .collect()
}
