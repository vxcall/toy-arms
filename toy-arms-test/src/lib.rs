use std::thread;
use std::ptr;
use std::time::Duration;
use toy_arms::VirtualKeyCode;

use winapi::shared::minwindef::LPVOID;
use winapi::um::fileapi::CreateFileW;
use winapi::{shared::{minwindef::DWORD, ntdef::LPCWSTR}, um::{consoleapi::AllocConsole, minwinbase::LPSECURITY_ATTRIBUTES, winnt::HANDLE, winuser::{MB_OK, MessageBoxA}}, um::wincon::FreeConsole};
use static_init::{dynamic};

toy_arms::create_entrypoint!(hack_main_thread);

type CreateFileWt = *const unsafe extern "system" fn(LPCWSTR, DWORD, DWORD, LPSECURITY_ATTRIBUTES, DWORD, DWORD, HANDLE) -> HANDLE;

const ORIGINAL_CREATE_FILEW: CreateFileWt = ptr::null_mut();
//#[dynamic]
//static mut original_create_filew:usize = 0;


unsafe extern "system" fn hook_create_filew(lp_file_name: LPCWSTR, dw_desired_access: DWORD, dw_share_mode: DWORD, lp_security_attributes: LPSECURITY_ATTRIBUTES,
dw_creation_disposition: DWORD, dw_flags_and_attributes: DWORD, h_template_file: HANDLE) -> HANDLE {
    (*(ORIGINAL_CREATE_FILEW))(
        lp_file_name, dw_desired_access, dw_share_mode, lp_security_attributes, dw_creation_disposition, dw_flags_and_attributes, h_template_file,
    )
}

fn hack_main_thread() {
    unsafe {
        let mut iatf = toy_arms::IatFinder::new("notepad.exe", "CreateFileW",
             hook_create_filew as LPVOID, ORIGINAL_CREATE_FILEW as LPVOID);
        iatf.run();
    }
    loop {
        if toy_arms::detect_keydown(VirtualKeyCode::VK_HOME) {
            break
        }
        thread::sleep(Duration::from_millis(50));
    }
}
