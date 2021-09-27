use std::thread;
use std::time::Duration;
use toy_arms::VirtualKeyCode;

use winapi::{
    um::consoleapi::AllocConsole, um::libloaderapi::GetModuleHandleA, um::wincon::FreeConsole,
};

toy_arms::create_entrypoint!(hack_main_thread);

fn hack_main_thread() {
    unsafe {
        AllocConsole();
    }
    unsafe {
        let iatf = toy_arms::IatFinder::new("notepad.exe".into(), "a".into());
        iatf.find_iat_entry();
    }
    loop {
        //let client = toy_arms::cast!(GetModuleHandleA("notepad.exe"), HINSTANCE -> usize);
        //*toy_arms::cast!(*client + 0x31ECB34, u8) = 5;
        if toy_arms::detect_keydown(VirtualKeyCode::VK_HOME) {
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }
    unsafe {
        FreeConsole();
    }
}
