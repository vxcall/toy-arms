use winapi::shared::minwindef::{LPCVOID, LPVOID};
use toy_arms::{MemoryEx, VirtualKeyCode};

fn main() {
    // Getting process information
    let memex = MemoryEx::from_process_name("csgo.exe");
    println!("process id = {}", memex.process_id);
    println!("process handle = {:?}", memex.process_handle);

    // You can get module information by using get_module_info
    let module_info = memex.get_module_info("client.dll").unwrap();
    loop {
        // read fetches the value at where the address is pointing.
        // U have to specify the type of the value with turbofish
        println!("{:?}", memex.read::<u32>(memex.get_module_base("client.dll").unwrap() + 0x31EDB20 as usize).unwrap());

        // Exit this loop by pressing INSERT
        if toy_arms::detect_keydown(VirtualKeyCode::VK_INSERT) {
            break;
        }
    }
}