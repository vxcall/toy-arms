use toy_arms::{MemoryEx, VirtualKeyCode};

fn main() {
    // This const has to be up to date.
    const DW_CLIENT_STATE: usize = 0x588FEC;
    const DW_CLIENT_STATE_STATE: usize = 0x108;
    const DW_FORCE_ATTACK: usize = 0x31EFD04;
    // Getting process information
    let memex = MemoryEx::from_process_name("csgo.exe");
    println!("process id = {}, \nprocess handle = {:?}", memex.process_id, memex.process_handle);

    // You can get module information by using get_module_info
    let module_info = memex.get_module_info("client.dll").unwrap();
    println!("{}", module_info.module_name);

    // read fetches the value at where the address is pointing.
    // U have to specify the type of the value with turbofish
    unsafe {
        println!("{:x}", memex.read::<i32>(memex.read::<u32>(memex.get_module_base("engine.dll").unwrap() + DW_CLIENT_STATE).unwrap() as usize + DW_CLIENT_STATE_STATE).unwrap());
    }

    loop {
        // write helps you tamper with the value.
        //memex.write::<u32>(memex.get_module_base("client.dll").unwrap() + DW_FORCE_ATTACK as usize, &mut 0x5).unwrap();

        // Exit this loop by pressing INSERT
        if toy_arms::detect_keydown(VirtualKeyCode::VK_INSERT) {
            break;
        }
    }
}