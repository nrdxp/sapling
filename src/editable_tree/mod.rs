//! Specification of an editable, undoable buffer of trees and some implementations thereof.

pub mod cursor_path;
pub mod dag;

use crate::arena::Arena;
use crate::ast::Ast;
use std::error::Error;

/// The possible ways you can move the cursor
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Prev,
    Next,
}

/// An enum to represent the two sides of a cursor
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Side {
    Prev,
    Next,
}

/// A trait specifying an editable, undoable buffer of trees
pub trait EditableTree<'arena, Node: Ast<'arena>>: Sized {
    type InsertError: Error;

    /* CONSTRUCTOR METHODS */

    /// Build a new `EditableTree`, given a tree
    fn new(arena: &'arena Arena<Node>, root: &'arena Node) -> Self;

    /* HISTORY METHODS */

    /// Move one step back in the tree history, returning `false` if there are no more changes
    fn undo(&mut self) -> bool;

    /// Move one step forward in the tree history, return `false` if there was no change to be
    /// redone
    fn redo(&mut self) -> bool;

    /* NAVIGATION METHODS */

    /// Returns a reference to the node that is currently the root of the AST.
    fn root(&self) -> &'arena Node;

    /// Returns the cursor node and its direct parent (if such a parent exists)
    fn cursor_and_parent(&self) -> (&'arena Node, Option<&'arena Node>);

    /// Returns a reference to the node that is currently under the cursor.
    fn cursor(&self) -> &'arena Node;

    /// Move the cursor in a given direction across the tree.  Returns [`Some`] error string if an
    /// error is found, or [`None`] if the movement was possible.
    fn move_cursor(&mut self, direction: Direction) -> Option<String>;

    /* EDIT METHODS */

    /// Updates the internal state so that the tree now contains `new_node` in the position of the
    /// `cursor`.
    fn replace_cursor(&mut self, new_node: Node);

    /// Updates the internal state so that the tree now contains `new_node` inserted as the first
    /// child of the selected node.  Also moves the cursor so that the new node is selected.
    fn insert_child(&mut self, new_node: Node) -> Result<(), Self::InsertError>;

    /// Updates the internal state so that the tree now contains `new_node` inserted as the first
    /// child of the selected node.  Also moves the cursor so that the new node is selected.
    fn insert_next_to_cursor(
        &mut self,
        new_node: Node,
        side: Side,
    ) -> Result<(), Self::InsertError>;

    /* DISPLAY METHODS */

    /// Build the text representation of the current tree into the given [`String`]
    fn write_text(&self, string: &mut String, format: &Node::FormatStyle);

    /// Build and return a [`String`] of the current tree
    fn to_text(&self, format: &Node::FormatStyle) -> String {
        let mut s = String::new();
        self.write_text(&mut s, format);
        s
    }
}
