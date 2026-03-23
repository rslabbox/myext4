//! `std::fs::File` backed block device implementation.
//!
//! This adapter is intended for local-image testing and tooling,
//! not for `no_std` kernel environments.

/// Placeholder type for a file-backed block device.
///
/// The concrete implementation will wrap a file descriptor and
/// perform block-aligned reads/writes.
pub struct FileBlockDevice;
