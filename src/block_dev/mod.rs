//! Block device abstraction layer.
//!
//! This module defines traits and common types for block-aligned I/O,
//! decoupling filesystem logic from concrete storage backends.

pub mod file_impl;

/// Common interface for a block device backend.
pub trait BlockDevice {
	/// Backend-specific error type.
	type Error;

	/// Read exactly one block into `buf` from `block_index`.
	fn read_block(&self, block_index: u64, buf: &mut [u8]) -> Result<(), Self::Error>;

	/// Write exactly one block from `buf` into `block_index`.
	fn write_block(&self, block_index: u64, buf: &[u8]) -> Result<(), Self::Error>;

	/// Return logical block size in bytes.
	fn block_size(&self) -> u32;
}
