use toy_arms::{detect_keydown, get_module_function_address, get_module_handle, HMODULE, Memory, VirtualKeyCode};
toy_arms::create_entrypoint!(hack_main_thread);

fn hack_main_thread() {
    let memory = Memory::from_module_name("client.dll");
    unsafe {
        let pattern = b"\xA0\x35\x05\x84\xB5\x14";
        memory.signature_scan(pattern, 0, 0);
    }
    loop {
        // To exit this hack loop when you input INSEERT KEY
        if detect_keydown(VirtualKeyCode::VK_INSERT) {
            break
        }
    }
}