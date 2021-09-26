use std::thread;
use std::time::Duration;
use toy_arms::VirtualKeyCode;

use bindings::Windows::Win32::Foundation::HINSTANCE;
use bindings::Windows::Win32::System::Console::{AllocConsole, FreeConsole};
use bindings::Windows::Win32::System::LibraryLoader::GetModuleHandleA;

toy_arms::create_entrypoint!(hack_main_thread);

fn hack_main_thread() {
    unsafe {
        AllocConsole();
    }

    loop {
        unsafe {
            let client = toy_arms::cast!(&GetModuleHandleA("client.dll"), HINSTANCE -> usize);

            *toy_arms::cast!(*client + 0x31ECB34, u8) = 5;
            if toy_arms::detect_keydown(VirtualKeyCode::VK_HOME) {
                break;
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
    unsafe {
        FreeConsole();
    }
}
