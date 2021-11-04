use toy_arms::{detect_keydown, Memory, VirtualKeyCode};
toy_arms::create_entrypoint!(hack_main_thread);

fn hack_main_thread() {
    let memory = Memory::from_module_name("client.dll");
    unsafe {
        //    let pattern = "46 83 ? 04 ? 44 ? 14 ? ? 45 FF";
        let pattern = "8D 34 85 ? ? ? ? 89 15 ? ? ? ? 8B 41 08 8B 48 04 83 F9 FF";
        match memory.signature_scan(pattern, 0, 0) {
            Some(i) => println!("Pattern found {:p}", i),
            None => println!("Pattern not found")
        }
    }
    loop {
        // To exit this hack loop when you input INSEERT KEY
        if detect_keydown(VirtualKeyCode::VK_INSERT) {
            break
        }
    }
}