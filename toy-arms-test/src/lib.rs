use std::ptr;
use std::time::Duration;
use toy_arms::{get_module_function_address, null_terminated_i8, VirtualKeyCode};

use winapi::shared::minwindef::UINT;
use winapi::shared::windef::HWND;
use winapi::{
    shared::ntdef::LPCSTR,
    um::{
        winuser::{MessageBoxA, MB_OK},
    },
};

use std::os::raw::c_int;
use detour::static_detour;
use winapi::shared::ntdef::LPCWSTR;
use winapi::um::winuser::MessageBoxW;

static_detour! {
    static MessageBoxAHook: unsafe extern "system" fn(HWND, LPCSTR, LPCSTR, UINT) -> c_int;
}

type MessageBoxAt = unsafe extern "system" fn(HWND, LPCSTR, LPCSTR, UINT) -> c_int;

struct HookArtifacts {
    target_fn: *mut std::ffi::c_void,
    detour_fn: *mut std::ffi::c_void,
    // *mut *mut std::ffi::c_void didnt work
    // MessageBoxAt didnt work as well
    original_fn: *mut std::ffi::c_void,
    released: bool
}

fn hook_messageboxa(hwnd: HWND, text: LPCSTR, caption: LPCSTR, utype: UINT) -> c_int {
    unsafe {
        MessageBoxAHook.call(hwnd, null_terminated_i8("This has been hacked"), caption, MB_OK)
    }
}

toy_arms::create_entrypoint!(hack_main_thread);
fn hack_main_thread() {
    unsafe {
        let address = get_module_function_address("USER32.dll", "MessageBoxA");
        let target: MessageBoxAt = std::mem::transmute(address);
        MessageBoxAHook
            .initialize(target, hook_messageboxa).expect("fuck")
            .enable().expect("ccc");
        MessageBoxA(ptr::null_mut(), null_terminated_i8("DEFAULT"), null_terminated_i8("DEFAULT"), MB_OK);
        MessageBoxAHook.disable().expect("not disabled");
    }
}