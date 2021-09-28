use std::thread;
use std::ptr;
use std::time::Duration;
use toy_arms::VirtualKeyCode;

use winapi::shared::minwindef::LPVOID;
use winapi::um::fileapi::CreateFileW;
use winapi::{shared::{minwindef::DWORD, ntdef::LPCWSTR}, um::{consoleapi::AllocConsole, minwinbase::LPSECURITY_ATTRIBUTES, winnt::HANDLE, winuser::{MB_OK, MessageBoxA}}, um::wincon::FreeConsole};
use static_init::{dynamic};
toy_arms::create_entrypoint!(hack_main_thread);

type CreateFileW_t = *const unsafe extern "system" fn(LPCWSTR, DWORD, DWORD, LPSECURITY_ATTRIBUTES, DWORD, DWORD, HANDLE) -> HANDLE;
//#[dynamic]
//static original_create_filew: CreateFileW_t = ptr::null_mut();


unsafe extern "system" fn hkCreateFileW(lp_file_name: LPCWSTR, dw_desired_access: DWORD, dw_share_mode: DWORD, lp_security_attributes: LPSECURITY_ATTRIBUTES,
dw_creation_disposition: DWORD, dw_flags_and_attributes: DWORD, h_template_file: HANDLE)  {
    MessageBoxA(ptr::null_mut(), "aaaaa".as_ptr() as _, "adfsa".as_ptr() as _, MB_OK);
    //original_create_filew(lp_file_name, dw_desired_access, dw_share_mode, lp_security_attributes, dw_creation_disposition, dw_flags_and_attributes, h_template_file)
}

fn hack_main_thread() {
    unsafe {
        AllocConsole();
    }
    unsafe {
        let mut iatf = toy_arms::IatFinder::new("notepad.exe".into(), "CreateFileW".into());
        iatf.run();
        println!("target entry = {:?}", iatf.target_entry);
        println!("void* CreateFileW = {:?}", CreateFileW as LPVOID);
        if *iatf.target_entry == CreateFileW as LPVOID {
        }
    }
    loop {
        //let client = toy_arms::cast!(GetModuleHandleA("notepad.exe"), HINSTANCE -> usize);
        //*toy_arms::cast!(*client + 0x31ECB34, u8) = 5;
        if toy_arms::detect_keydown(VirtualKeyCode::VK_HOME) {
            break
        }
        thread::sleep(Duration::from_millis(50));
    }
    unsafe {
        FreeConsole();
    }
}
