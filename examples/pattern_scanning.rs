/*
This is an example to demonstrate how to use powerful pattern scan feature in toy-arms.
Make sure you inject this image to csgo.exe.
The model pattern is for dwForceAttack.
*/

use std::thread::sleep;
use toy_arms::{detect_keydown, Module, pattern_scan_all_modules, VirtualKeyCode};
toy_arms::create_entrypoint!(hack_main_thread);

fn hack_main_thread() {
    let memory = Module::from_module_name("client.dll").unwrap();
    let dw_force_attack_pattern = "89 0D ? ? ? ? 8B 0D ? ? ? ? 8B F2 8B C1 83 CE 04";

    match memory.find_pattern(dw_force_attack_pattern) {
        Some(i) => println!("offset: 0x{:x}", i),
        None => println!("Pattern not found")
    }

    match pattern_scan_all_modules(dw_force_attack_pattern, 2, 0) {
        Ok(e) => {
            println!("RESULT");
            println!("address: {:x}", e.0);
            println!("module name: {}", e.1);
        },
        _ => println!("Not Found")
    }

    loop {
        // To exit this hack loop when you input INSEERT KEY
        if detect_keydown(VirtualKeyCode::VK_INSERT) {
            break
        }
    }
}