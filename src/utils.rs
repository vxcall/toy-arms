//! utils contains functions that is used across this entire crate multiple times because of its usefulness.

use std::str::Utf8Error;

#[doc(hidden)]
pub use winapi::{
    shared::minwindef::{FARPROC, HMODULE},
    shared::minwindef::BOOL, shared::minwindef::HINSTANCE, shared::minwindef::TRUE,
    um::consoleapi::AllocConsole, um::libloaderapi::DisableThreadLibraryCalls,
    um::wincon::FreeConsole, um::winnt::DLL_PROCESS_ATTACH,
    um::libloaderapi::{GetModuleHandleA, GetProcAddress},
};

/// cast is a substitution of reinterpret_cast in C++.
/// * `$address` - address or variable you wanna cast.
/// * `$type` - type you want to cast $address into.
#[macro_export]
macro_rules! cast {
    // Value cast
    (mut $address:expr, $type:ident) => {
        $address as *mut $type
    };
    ($address:expr, $type:ident) => {
        $address as *const $type
    };
}

/// create_entrypoint fully automates the process of making DllMain on your behalf.
/// * `function` - function you want to run in the newly created thread.
#[macro_export]
macro_rules! create_entrypoint {
    ($function:expr) => {
        #[no_mangle]
        #[allow(non_snake_cake)]
        extern "system" fn DllMain(h_module: $crate::HINSTANCE, dw_reason: u32, _: *const ::std::ffi::c_void, ) -> $crate::BOOL {
            if dw_reason == $crate::DLL_PROCESS_ATTACH {
                unsafe {
                    $crate::DisableThreadLibraryCalls(h_module);
                }
                ::std::thread::spawn(|| {
                    if cfg!(debug_assertions) {
                        unsafe { $crate::AllocConsole(); }
                    }
                    $function();
                    if cfg!(debug_assertions) {
                        unsafe { $crate::FreeConsole(); }
                    }
                });
            }
            $crate::TRUE
        }
    };
}

/// get_module_handle returns Option where should contains HMODULE.
/// This function may fail even if you put a correct dll name somehow, so it tries get handle for 100 times then returns Option
/// * `text` - name of the module you want
pub fn get_module_handle(module_name: &str) -> Option<HMODULE> {
    unsafe {
        let mut module_handle: HMODULE = GetModuleHandleA(make_lpcstr(module_name));
        for _ in 0..100 {
            if module_handle != 0 as HMODULE { break }
            module_handle = GetModuleHandleA(make_lpcstr(module_name));
        }
        if module_handle == 0 as HMODULE {
            return None
        } else {
            Some(module_handle)
        }
    }
}

pub(crate) fn make_lpcstr(text: &str) -> *const i8 {
    format!("{}{}", text, "\0").as_ptr() as *const i8
}

pub(crate) unsafe fn read_null_terminated_string(base_address: usize) -> Result<String, Utf8Error> {
    let mut name: Vec<u8> = Vec::new();
    let mut i: isize = 0;
    loop {
        let char_as_u8 = *(base_address as *const u8).offset(i);
        if char_as_u8 == 0 {
            return Ok(std::str::from_utf8(&name[..])?.to_owned());
        }
        name.push(char_as_u8);
        i += 1;
    }
}