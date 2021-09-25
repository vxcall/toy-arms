pub use bindings::Windows::Win32::Foundation::{BOOL, HINSTANCE};
pub use bindings::Windows::Win32::System::LibraryLoader::DisableThreadLibraryCalls;
pub use bindings::Windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
pub use std::ffi::c_void;
pub use std::thread;
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
        extern "system" fn DllMain(
            h_module: $crate::HINSTANCE,
            dw_reason: u32,
            _: *const ::std::ffi::c_void,
        ) -> $crate::BOOL {
            if dw_reason == $crate::DLL_PROCESS_ATTACH {
                unsafe {
                    $crate::DisableThreadLibraryCalls(h_module);
                }
                ::std::thread::spawn(|| $function());
            }
            $crate::BOOL(1)
        }
    };
}
