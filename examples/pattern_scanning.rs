/*
This is an example to demonstrate how to use powerful pattern scan feature in toy-arms.
Make sure you inject this image to csgo.exe.
The model pattern is for dwForceAttack.
*/

use toy_arms::{detect_keydown, Memory, VirtualKeyCode};
toy_arms::create_entrypoint!(hack_main_thread);

fn hack_main_thread() {
    let memory = Memory::from_module_name("client.dll");
    //    let pattern = "46 83 ? 04 ? 44 ? 14 ? ? 45 FF";
    let dw_force_attack_pattern = "89 0D ? ? ? ? 8B 0D ? ? ? ? 8B F2 8B C1 83 CE 04";
    match memory.pattern_scan(dw_force_attack_pattern, 2, 0) {
        Some(i) => println!("Pattern found at: {:x}", i),
        None => println!("Pattern not found")
    }
    loop {
        // To exit this hack loop when you input INSEERT KEY
        if detect_keydown(VirtualKeyCode::VK_INSERT) {
            break
        }
    }
}