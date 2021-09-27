pub use winapi::{
    shared::minwindef::BOOL, shared::minwindef::HINSTANCE, shared::minwindef::TRUE,
    um::libloaderapi::DisableThreadLibraryCalls, um::winnt::DLL_PROCESS_ATTACH,
};

/// cast is a substitution of reinterpret_cast in C++.
/// * `$address` - address or variable you wanna cast.
/// * `$type` - type you want to cast $address into.
#[macro_export]
macro_rules! cast {
    // Value cast
    ($address:expr, $type:ident) => {
        $address as *mut $type
    };
}

/// create_entrypoint fully automates the process of making DllMain on your behalf.
/// * `function` - function you want to run in the newly created thread.
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
            $crate::TRUE
        }
    };
}

#[macro_export]
macro_rules! null_terminated {
    ($str:expr) => {
        format!("{}{}", $str, "\0").as_ptr() as *const i8
    };
}
