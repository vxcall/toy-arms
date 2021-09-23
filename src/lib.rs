use bindings::Windows::Win32::Foundation::{BOOL, HINSTANCE};
use bindings::Windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls;
use bindings::Windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use std::ffi::c_void;
use std::thread;

#[macro_export]
macro_rules! cast {
    ($val:expr, $from:ident -> $to:ident) => {
        $val as *const $from as *const $to
    };
    ($val:expr, mut $from:ident -> $to:ident) => {
        mut $val as *mut $from as *mut $to
    };
}

#[macro_export]
macro_rules! create_entrypoint {
    ($function:expr) => {
        #[no_mangle]
        #[allow(non_snake_cake)]
        extern "system" fn DllMain(h_module: HINSTANCE, dw_reason: u32, _: *const c_void) -> BOOL {
            if dw_reason == DLL_PROCESS_ATTACH {
                unsafe {
                    DisableThreadLibraryCalls(h_module);
                }
                thread::spawn(|| $function());
            }
            BOOL(1)
        }
    };
}
