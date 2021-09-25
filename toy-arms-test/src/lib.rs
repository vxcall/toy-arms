use bindings::Windows::Win32::Foundation::HINSTANCE;
use bindings::Windows::Win32::System::Console::{AllocConsole, FreeConsole};
use bindings::Windows::Win32::System::LibraryLoader::GetModuleHandleA;
use bindings::Windows::Win32::UI::KeyboardAndMouseInput::GetAsyncKeyState;
use toy_arms;

const VK_INSERT: i32 = 0x2d;

fn hack_main_thread() {
    unsafe {
        AllocConsole();
    }

    loop {
        unsafe {
            let client = toy_arms::cast!(&GetModuleHandleA("client.dll"), HINSTANCE -> usize);

            let p_dw_force_attack = toy_arms::cast!(*client + 0x31ECB34, usize -> u8);
            println!("{:?}", *p_dw_force_attack);
            if GetAsyncKeyState(VK_INSERT) & 1 != 0 {
                break;
            }
        }
    }
    unsafe {
        FreeConsole();
    }
}

toy_arms::create_entrypoint!(hack_main_thread);
