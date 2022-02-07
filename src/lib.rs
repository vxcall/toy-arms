#![doc(html_logo_url = "https://svgshare.com/i/cF0.svg")]

#[cfg(target_os = "windows")]
mod keyboard;

#[doc(inline)]
pub use keyboard::*;

#[cfg(feature = "internal")]
pub mod internal;

#[cfg(feature = "external")]
pub mod external;

// This is necessary to let submodules import the functions in this.
pub mod pattern_scan_common;

#[doc(inline)]
pub use internal::utils::*;

pub trait GameObject {
    unsafe fn from_raw(address: *const usize) -> Option<*mut Self>;
}
