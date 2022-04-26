use toy_arms::{ VirtualKeyCode };
use toy_arms::external::Process;

const DW_FORCE_ATTACK_PATTERN: &str = "8D 34 85 ? ? ? ? 89 15 ? ? ? ? 8B 41 08 8B 48 04 83 F9 FF";

fn main() {
    let mut once = false;

    // Getting process information
    let process = Process::from_process_name("csgo.exe").unwrap();
    // You can get module information by using get_client
    let client = process.get_module_info("client.dll").unwrap();
    println!("{:#?}", client);


    let address = client.find_pattern(DW_FORCE_ATTACK_PATTERN);
    match address {
        Some(i) => println!("found *dwForceAttack pattern at 0x{:x}", i),
        None => println!("NOTHING FOUND"),
    }

    let offset = client.pattern_scan(
        DW_FORCE_ATTACK_PATTERN,
        2,
        0,
    );
    match offset {
        Some(i) => println!("found dwForceAttack offset at 0x{:x}", i),
        None => println!("NOTHING FOUND"),
    }

    loop {
        if !once {
            println!("Press INSERT to exit...");
            once = !once;
        }
        // Exit this loop by pressing INSERT
        if toy_arms::detect_keydown!(VirtualKeyCode::VK_INSERT) {
            break;
        }
    }
}
