use internal::pattern_scan_all_modules;
use toy_arms::utils::keyboard::VirtualKeyCode;

internal::create_entrypoint!(hack_main);

fn hack_main() {
    let mut once = false;

    let dw_force_attack_pattern = "89 0D ? ? ? ? 8B 0D ? ? ? ? 8B F2 8B C1 83 CE 04";
    match pattern_scan_all_modules(dw_force_attack_pattern) {
        Some(e) => {
            println!("[+] address found: {:x}", e.0);
            println!("[+] module name: {}", e.1);
        }
        _ => println!("Not Found"),
    }

    loop {
        if !once {
            println!("[+] Press INSERT to exit...");
            once = !once;
        }
        if toy_arms::utils::keyboard::detect_keypress(VirtualKeyCode::VK_INSERT) {
            break;
        }
    }
}
