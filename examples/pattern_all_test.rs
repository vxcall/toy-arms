use toy_arms::VirtualKeyCode;
toy_arms::create_entrypoint!(hack_main);

fn hack_main() {
    let unknown_pattern = "55 48 8B EC 48 81 EC B0 00 00 00 48 89 4D 98";
    let result = toy_arms::pattern_scan_specific_range(unknown_pattern, 0x10000000000, 0x30000000000).unwrap();
    println!("{:p}", result);
    loop {
        if toy_arms::detect_keydown(VirtualKeyCode::VK_INSERT) {
            break;
        }
    }
}
