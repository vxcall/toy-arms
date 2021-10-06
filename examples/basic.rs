use toy_arms::{detect_keydown, get_module_function_address, get_module_handle, VirtualKeyCode};
use toy_arms::cast;

toy_arms::create_entrypoint!(hack_main_thread);

fn hack_main_thread() {
    // Gets module handle
    let module_handle = get_module_handle("client.dll");
    println!("{:?}", module_handle as usize);

    unsafe {
        // Gets function address
        let function_address = get_module_function_address("USER32.dll", "MessageBoxA");
        println!("{:?}", function_address);
    }

    const DW_FORCE_ATTACK: usize = 0x31ECB34;
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
