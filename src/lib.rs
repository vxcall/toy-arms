
#[cfg(target_os = "windows")]
mod keyboard;
#[doc(inline)]
pub use keyboard::*;

mod memory;
#[doc(inline)]
pub use memory::*;

mod memory_ex;
#[doc(inline)]
pub use memory_ex::*;

mod pattern_scan_core;

mod utils;
#[doc(inline)]
pub use utils::*;

pub trait GameObject {
    unsafe fn from_raw(address: *const usize) -> Option<*mut Self>;
}
