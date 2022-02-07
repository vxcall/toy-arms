use toy_arms::{ VirtualKeyCode };
use toy_arms::external::Process;

fn main() {
    // Getting process information
    let process = Process::from_process_name("csgo.exe");

    // You can get module information by using get_module_info
    let module_info = process.get_module_info("client.dll").unwrap();

    let address = module_info.find_pattern("89 0D ? ? ? ? 8B 0D ? ? ? ? 8B F2 8B C1 83 CE 04");
    println!("0x{:x}", address.unwrap());

    loop {
        // Exit this loop by pressing INSERT
        if toy_arms::detect_keydown!(VirtualKeyCode::VK_INSERT) {
            break;
        }
    }
    println!("program ends");
}
