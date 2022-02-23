/*
This is an example to demonstrate how to use powerful pattern scan feature in toy-arms.
Make sure you inject this image to csgo.exe.
The model pattern is for dwForceAttack.
*/

use toy_arms::{
    detect_keypress,
    internal::{
        Module
    },
    VirtualKeyCode
};
toy_arms::create_entrypoint!(hack_main_thread);

fn hack_main_thread() {
    let mut once = false;

    let client = Module::from_module_name("client.dll").unwrap();
    let dw_force_attack_pattern = "8D 34 85 ? ? ? ? 89 15 ? ? ? ? 8B 41 08 8B 48 04 83 F9 FF";

    match client.find_pattern(dw_force_attack_pattern) {
        Some(i) => println!("address: 0x{:x}", i),
        None => println!("Pattern not found"),
    }

    match client.pattern_scan(
        "89 0D ? ? ? ? 8B 0D ? ? ? ? 8B F2 8B C1 83 CE 04",
        2,
        0,
    ) {
        Some(i) => println!("address: 0x{:x}", i),
        None => println!("Offset not found"),
    }

    loop {
        if !once {
            println!("Press INSERT to exit...");
            once = !once;
        }
        // To exit this hack loop when you input INSEERT KEY
        if detect_keypress(VirtualKeyCode::VK_INSERT) {
            break;
        }
    }
}
