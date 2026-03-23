//! Pure ext4 on-disk specification layer.
//!
//! This layer contains only structure/layout definitions and constants.
//! Parsing and validation logic belongs to `core`.

pub mod dir_entry;
pub mod extent;
pub mod group_desc;
pub mod inode;
pub mod superblock;
