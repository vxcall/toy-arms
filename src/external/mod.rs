#[cfg(feature = "external")]
pub mod external;
#[cfg(feature = "external")]
mod pattern_scan;
#[cfg(feature = "external")]
pub mod error;

#[cfg(feature = "external")]
pub use external::*;
