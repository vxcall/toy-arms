/*
This is the demonstration of how to use internal analysis feature in toy-arms.
This code gets module handle and function address of the func called MessageBoxA as an example.
Then read the value called dwForceAttack and overwrite it to make player shoot.
The offset DW_FORCE_ATTACK works as of the day i wrote this but it might not be up to date in your case.
*/

use toy_arms::{detect_keypress, detect_keydown, get_module_handle, HMODULE, VirtualKeyCode};
use toy_arms::cast;
use toy_arms::internal::{get_module_function_address, Module};

toy_arms::create_entrypoint!(hack_main_thread);

// This offset has to be up to date.
const DW_FORCE_ATTACK: usize = 0x31EFCB4;

fn hack_main_thread() {
    // Gets module handle
    let module_handle: HMODULE = get_module_handle("client.dll").unwrap();
    println!("module handle = {:?}", module_handle as usize);

    unsafe {
        // Gets function address
        let function_address = get_module_function_address("USER32.dll", "MessageBoxA").unwrap();
        println!("function address = {:?}", function_address);
    }

    let module = Module::from_module_name("client.dll").unwrap();

    let shoot_flag = cast!(mut module_handle as usize + DW_FORCE_ATTACK, u8);

    loop {
        unsafe {
            // Auto shoot
            *shoot_flag = 5u8;
        }
        // To exit this hack loop when you input INSEERT KEY
        if detect_keypress(VirtualKeyCode::VK_INSERT) {
            break;
        }

        // just flexing this neat function xd.
        if detect_keydown!(VirtualKeyCode::VK_HOME) {
            println!("HOME is both pressed");
        }
    }
}
