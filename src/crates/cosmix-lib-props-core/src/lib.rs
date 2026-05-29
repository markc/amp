//! Pure-type core of the SPEC 07 property surface.
//!
//! This crate carries `PropPath`, `PropValue`, `PropTree`,
//! `PropDescribe`, `redact`, `diff` — the read-side types every
//! `<svc>.props.*` consumer needs without taking on the substrate
//! machinery (NamespaceSpec, hooks, runtime, storage backends, audit
//! HMAC, SPEC 12 mutation router) that lives in the sibling
//! `cosmix-lib-props-store` crate (cos repo).
//!
//! The default feature set ships only the pure-type surface. Enable
//! the opt-in `amp` feature to pull `cosmix-lib-amp` + `chrono` and
//! gain the `amp::dispatch_props` (SPEC 07 §2 read wire) and
//! `publish::*` (SPEC 07 §3/§4 wire builders) modules.

pub mod describe;
pub mod diff;
pub mod path;
pub mod redact;
pub mod tree;
pub mod value;

#[cfg(feature = "amp")]
pub mod amp;
#[cfg(feature = "amp")]
pub mod publish;

pub use describe::{PropDescribe, PropType};
pub use diff::diff;
pub use path::{PropPath, PropPathError};
pub use redact::redact;
pub use tree::PropTree;
pub use value::PropValue;
