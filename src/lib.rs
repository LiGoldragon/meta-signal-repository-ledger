//! Meta Signal contract for repository-ledger authority and configuration.
//!
//! The contract carries privileged operations only: repository registration
//! and retirement, the fallback spool directory policy, and per-repository
//! mirror policy. Event ingestion, storage and the read queries live in
//! `signal-repository-ledger` and in the `repository-ledger` runtime.
//!
//! `ethos/signal.ethos` is the schema authority; `build.rs` checks the
//! checked-in Rust projection in `src/generated/signal.rs` against it. The
//! portable rkyv frame, its kinds, and the wire framing come from `signal`.

pub mod generated;
pub use generated::signal::*;

pub use signal_repository_ledger::{Class, FilesystemPath, Name, Registration};

/// The authored Ethos source of this contract.
pub const ETHOS: &str = include_str!("../ethos/signal.ethos");
/// The Rust projection generated from [`ETHOS`].
pub const ETHOS_RUST: &str = include_str!("generated/signal.rs");
