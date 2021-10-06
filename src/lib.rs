#[cfg(target_os = "windows")]
mod keyboard;
#[doc(inline)]
pub use keyboard::*;

mod utils;

mod memory;
#[doc(inline)]
pub use memory::*;

#[doc(inline)]
pub use self::utils::*;

pub trait GameObject {
    unsafe fn from_raw(address: *const usize) -> Option<*mut Self>;

}