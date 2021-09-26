pub use bindings::{DisableThreadLibraryCalls, BOOL, DLL_PROCESS_ATTACH, HINSTANCE};
pub mod vk;

/// cast is a substitution of reinterpret_cast in C++.
/// * `$val` - address or variable you wanna cast.
/// * `$from` - type of the variable you passed in $val.
/// * `$to` - type you want to cast $val into.
#[macro_export]
macro_rules! cast {
    ($val:expr, $from:ident -> $to:ident) => {
        $val as *const $from as *const $to
    };
    ($val:expr, mut $from:ident -> $to:ident) => {
        $val as *mut $from as *mut $to
    };
    ($val:expr, $type:ident) => {
        $val as *mut $type
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
            $crate::BOOL(1)
        }
    };
}

pub use self::keyboard::detect_keydown;
pub mod keyboard {
    use bindings::GetAsyncKeyState;

    /// detect_keydown returns true when user pressed the specified key.
    pub fn detect_keydown(vk_code: i32) -> bool {
        if unsafe { GetAsyncKeyState(vk_code) } & 1 != 0 {
            true
        } else {
            false
        }
    }
}
