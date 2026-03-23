//! High-level filesystem operations.
//!
//! This layer provides path-oriented access, file operations, and
//! optional journaling integration on top of `core`.

pub mod directory;
pub mod file;
pub mod journal;
