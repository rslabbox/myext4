//! Space allocation internals for writable operations.
//!
//! This module is responsible for free-space discovery and block/inode
//! reservation strategies.

pub mod bitmap;
pub mod buddy;
