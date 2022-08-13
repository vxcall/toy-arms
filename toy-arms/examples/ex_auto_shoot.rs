/*
This is the demonstration of how to use external feature of toy-arms.
Following code is trying to get process id and process handle first, then getting a value called dwClientState_state.
Then showing the way to overwrite value at dwForceAttack to make player shoot.
The offset DW_CLIENT_STATE, DW_CLIENT_STATE_STATE and DW_FORCE_ATTACK work as of the day i wrote this but it might not be up to date in your case.
*/

use toy_arms::external::process::Process;
use toy_arms::external::{read, write};
use toy_arms::utils::keyboard::VirtualKeyCode;

fn main() {
    // This const has to be up to date.
    const DW_FORCE_ATTACK: u32 = 0x320BDE8;
    // Getting process information
    let process = Process::from_process_name("csgo.exe").unwrap();
    println!(
        "[+] process id: {}, \n[+] process handle: {:?}",
        process.id, process.handle
    );

    // You can get module information by using get_module_info
    let module_info = process.get_module_info("client.dll").unwrap();
    println!("[+] module name: {}", module_info.name);

    loop {
        // write helps you tamper with the value.
        write::<u32>(
            &process.handle,
            process.get_module_base("client.dll").unwrap() + DW_FORCE_ATTACK as usize,
            &mut 0x5,
        )
        .unwrap();

        // Exit this loop by pressing INSERT
        if toy_arms::utils::keyboard::detect_keypress(VirtualKeyCode::VK_INSERT) {
            break;
        }
    }
}
