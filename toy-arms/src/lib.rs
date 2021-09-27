#[cfg(target_os = "windows")]
use std::ffi::c_void;
mod iat_hook;
pub use iat_hook::IatFinder;

mod keyboard;
#[doc(inline)]
pub use keyboard::*;

mod macro_utils;
#[doc(inline)]
pub use self::macro_utils::*;
