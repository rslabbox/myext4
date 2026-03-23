//! ext4-rs crate root.
//!
//! Layered modules:
//! - block_dev: device abstraction and adapters.
//! - spec: on-disk data structure definitions only.
//! - core: parsing, validation, lookup, and allocation internals.
//! - fs: higher-level filesystem operations.
//! - vfs: external-facing integration interface.
//! - utils: shared helpers.
#![no_std]

pub mod block_dev;
pub mod core;
pub mod fs;
pub mod spec;
pub mod utils;
pub mod vfs;
