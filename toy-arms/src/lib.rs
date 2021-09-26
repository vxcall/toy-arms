use std::ffi::c_void;
mod iat_hook;

mod keyboard;
#[doc(inline)]
pub use keyboard::*;

mod macro_utils;
#[doc(inline)]
pub use self::macro_utils::*;

type LPVOID = *mut c_void;
