use std::str::Utf8Error;
use std::collections::HashMap;

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

pub fn get_module_handle(text: &str) -> HMODULE {
    unsafe {
        let mut module_handle: HMODULE = GetModuleHandleA(make_lpcstr(text));
        loop {
            if module_handle != 0 as HMODULE { break module_handle }
            module_handle = GetModuleHandleA(make_lpcstr(text));
        }
    }
}

pub unsafe fn get_module_function_address(module_name: &str, function_name: &str) -> FARPROC {
    GetProcAddress(get_module_handle(module_name), make_lpcstr(function_name))
}

fn make_lpcstr(text: &str) -> *const i8 {
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

pub(crate) unsafe fn signature_scan_core(base: *mut u8, end: usize, pattern: &[u8], _offset: i32, _extra: i32) -> Option<*mut u8> {
    let right_most_wildcard_index = match get_right_most_wildcard(pattern){
        Some(i) => i,
        None => pattern.len()
    };
    let bmt = build_bad_match_table(pattern, right_most_wildcard_index);

    let mut current = (base as *mut u8).offset(pattern.len() as isize - 1 as isize);

    while (current as usize) < end {
        for (i, p) in pattern.iter().rev().enumerate() {
            // if pattern == current or pattern == ?, then
            if *p == b'\x3F' || *p == *current {
                if *p == pattern[0] {
                    // This is fired when the pattern is found.
                    return Some(current);
                }
                current = current.offset(-1);
            // if pattern != current
            } else {
                let movement_num = if let Some(i) = bmt.get(&*current) {
                    i.clone()
                } else { right_most_wildcard_index };
                current = current.offset(movement_num as isize + i as isize);
                break;
            }
        }
    }
    None
}

fn build_bad_match_table(pattern: &[u8], right_most_wildcard_index: usize) -> HashMap<&u8, usize> {
    let mut bad_match_table = HashMap::new();
    let pattern_length = pattern.len();
    for (i, p) in pattern.iter().enumerate() {
        let table_value = (pattern_length as isize - i as isize - 2) as usize;
        // if right_most_wildcard_index is pattern.len(), it's gonna be classified to else block anytime.
        let table_value = if table_value > right_most_wildcard_index {
            right_most_wildcard_index + 1
        } else if table_value < 1 {
            1
        } else {
            table_value
        };
        bad_match_table.insert(p, table_value);
    }
    bad_match_table
}

/// get_right_most_wildcard seeks the position of right most question mark and returns its index.
fn get_right_most_wildcard(pattern: &[u8]) -> Option<usize> {
    for (i, p) in pattern.iter().enumerate() {
        // \x3F represents '?' in ASCII table.
        if *p == b'\x3F' {
            return Some(i);
        }
    }
    None
}