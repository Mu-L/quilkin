//! Well known dynamic metadata used by Quilkin.

/// The default key under which the [`super::capture_bytes`] filter puts the
/// byte slices it extracts from each packet.
/// - **Type** `Vec<u8>`
pub const CAPTURED_BYTES: &str = "quilkin.dev/captured_bytes";
