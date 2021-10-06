use toy_arms::{create_entrypoint, cast, VirtualKeyCode};
use toy_arms::Memory;
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

fn hack_main_thread() {
    let memory = Memory::from_module("client.dll");
    unsafe {
        //let dw_local_player = memory.read_mut::<LocalPlayer>(0xDA244C);
        loop {
            if let Some(i) = LocalPlayer::from_raw(memory.read_mut(0xDa244c)) {
                println!("health = {:?}", (*i).get_health());
            };
            if toy_arms::detect_keydown(VirtualKeyCode::VK_INSERT) {
                break;
            }
        }
    }

}