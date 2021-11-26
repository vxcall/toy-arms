#![doc(html_logo_url = "https://svgshare.com/i/cF0.svg")]

#[cfg(target_os = "windows")]
mod keyboard;
#[doc(inline)]
pub use keyboard::*;

mod internal;
#[doc(inline)]
pub use internal::*;

mod external;
#[doc(inline)]
pub use external::*;

mod pattern_scan_core;

mod utils;
#[doc(inline)]
pub use utils::*;

pub trait GameObject {
    unsafe fn from_raw(address: *const usize) -> Option<*mut Self>;
}
