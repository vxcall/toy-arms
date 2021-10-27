use toy_arms::{detect_keydown, get_module_function_address, get_module_handle, HMODULE, Memory, VirtualKeyCode};
use toy_arms::cast;

toy_arms::create_entrypoint!(hack_main_thread);

// This offset has to be up to date.
const DW_FORCE_ATTACK: usize = 0x31EFCB4;

fn hack_main_thread() {
    // Gets module handle
    let mut module_handle: HMODULE = 0 as HMODULE;
    while module_handle == 0 as HMODULE {
        module_handle = get_module_handle("client.dll");
    };
    println!("module handle = {:?}", module_handle as usize);

    unsafe {
        // Gets function address
        let function_address = get_module_function_address("USER32.dll", "MessageBoxA");
        println!("function address = {:?}", function_address);
    }

    let memory = Memory::from_module_name("client.dll");

    let shoot_flag = cast!(mut module_handle as usize + DW_FORCE_ATTACK, u8);
    loop {
        unsafe {
            // Auto shoot
            *shoot_flag = 5u8;
        }
        // To exit this hack loop when you input INSEERT KEY
        if detect_keydown(VirtualKeyCode::VK_INSERT) {
            break
        }
    }
}
