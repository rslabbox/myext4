//! Core ext4 logic layer.
//!
//! Implements parsing, validation, address translation, and low-level
//! data operations based on `spec` structures.

pub mod allocator;
pub mod extent_tree;
pub mod inode_table;
pub mod superblock;
