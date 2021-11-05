/*
This example is the demonstration of getting player health with toy-arms internal memory analysis feature.
Make sure that you inject this image to csgo.exe.
also, the offset of DW_LOCAL_PLAYER works as of the day i wrote this but it might not be up to date in your case.
*/
use toy_arms::{create_entrypoint, cast, VirtualKeyCode};
use toy_arms::Module;
use toy_arms_derive::GameObject;
use toy_arms::GameObject;

create_entrypoint!(hack_main_thread);

#[derive(GameObject)]
struct LocalPlayer {
    pointer: *const usize, // Denotes the base address of LocalPlayer
}

impl LocalPlayer {
    unsafe fn get_health(&self) -> u16 {
        *cast!(self.pointer as usize + 0x100, u16)
    }
}

// This offset has to be up to date.
const DW_LOCAL_PLAYER: i32 = 0xDA545C;

fn hack_main_thread() {
    let memory = Module::from_module_name("client.dll");
    unsafe {
        //let dw_local_player = memory.read_mut::<LocalPlayer>(0xDA244C);
        loop {
            if let Some(i) = LocalPlayer::from_raw(memory.read_mut(DW_LOCAL_PLAYER)) {
                println!("health = {:?}", (*i).get_health());
            };
            if toy_arms::detect_keydown(VirtualKeyCode::VK_INSERT) {
                break;
            }
        }
    }

}