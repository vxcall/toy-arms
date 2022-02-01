#![doc(html_logo_url = "https://svgshare.com/i/cF0.svg")]

#[cfg(target_os = "windows")]
mod keyboard;
#[doc(inline)]
pub use keyboard::*;

#[cfg(feature = "internal")]
mod internal;
#[doc(inline)]
#[cfg(feature = "internal")]
pub use internal::*;

#[cfg(feature = "external")]
mod external;
#[doc(inline)]
#[cfg(feature = "external")]
pub use external::*;

#[cfg(feature = "internal")]
mod pattern_scan_core;

mod utils;
#[doc(inline)]
pub use utils::*;

pub trait GameObject {
    unsafe fn from_raw(address: *const usize) -> Option<*mut Self>;
}
