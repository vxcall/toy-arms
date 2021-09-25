use std::thread;
use std::time::Duration;
use toy_arms;

use bindings::Windows::Win32::Foundation::HINSTANCE;
use bindings::Windows::Win32::System::Console::{AllocConsole, FreeConsole};
use bindings::Windows::Win32::System::LibraryLoader::GetModuleHandleA;

fn hack_main_thread() {
    unsafe {
        AllocConsole();
    }

    loop {
        unsafe {
            let client = toy_arms::cast!(&GetModuleHandleA("client.dll"), HINSTANCE -> usize);

            let p_dw_force_attack = toy_arms::cast!(*client + 0x31ECB34, usize -> u8);
            println!("{:?}", *p_dw_force_attack);
            if toy_arms::detect_keydown(toy_arms::vk::VK_HOME) {
                break;
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
    unsafe {
        FreeConsole();
    }
}

toy_arms::create_entrypoint!(hack_main_thread);
