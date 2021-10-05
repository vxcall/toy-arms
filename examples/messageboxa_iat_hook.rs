use std::ptr;
use toy_arms::{get_module_function_address, make_lpcstr};

use winapi::shared::minwindef::UINT;
use winapi::shared::windef::HWND;
use winapi::{
    um::{
        winnt::LPCSTR,
        winuser::{MessageBoxA, MB_OK},
    },
};

use std::os::raw::c_int;
use detour::static_detour;

static_detour! {
    static MessageBoxAHook: unsafe extern "system" fn(HWND, LPCSTR, LPCSTR, UINT) -> c_int;
}

type MessageBoxAt = unsafe extern "system" fn(HWND, LPCSTR, LPCSTR, UINT) -> c_int;

fn hook_messageboxa(hwnd: HWND, _text: LPCSTR, caption: LPCSTR, utype: UINT) -> c_int {
    unsafe {
        MessageBoxAHook.call(hwnd, make_lpcstr("This has been hacked"), caption, utype)
    }
}

toy_arms::create_entrypoint!(hack_main_thread);
fn hack_main_thread() {
    unsafe {
        let address = get_module_function_address("USER32.dll", "MessageBoxA");
        let target: MessageBoxAt = std::mem::transmute(address);
        MessageBoxAHook
            .initialize(target, hook_messageboxa).unwrap()
            .enable().unwrap();
        MessageBoxA(ptr::null_mut(), make_lpcstr("DEFAULT"), make_lpcstr("DEFAULT"), MB_OK);
        MessageBoxAHook.disable().unwrap();
    }
}
