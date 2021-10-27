use toy_arms::{detect_keydown, get_module_function_address, get_module_handle, HMODULE, Memory, VirtualKeyCode};
toy_arms::create_entrypoint!(hack_main_thread);

fn hack_main_thread() {
    let memory = Memory::from_module_name("client.dll");
    unsafe {
        let pattern = b"\x85\xC9\x74\x1A??\x24\xBC???\x04\x75";
        println!("pattern found: {:p}", memory.signature_scan(pattern, 0, 0).unwrap());
    }
    loop {
        // To exit this hack loop when you input INSEERT KEY
        if detect_keydown(VirtualKeyCode::VK_INSERT) {
            break
        }
    }
}