use winapi::shared::minwindef::LPVOID;
use toy_arms::{create_entrypoint, cast, VirtualKeyCode};
use toy_arms::Memory;

create_entrypoint!(hack_main_thread);

struct LocalPlayer;

impl LocalPlayer {
    unsafe fn get_health(&self) -> u16 {
        *cast!(*(self as *const LocalPlayer as *const usize) + 0x100, u16)
    }
}


fn hack_main_thread() {
    let memory = Memory::from_module("client.dll");
    unsafe {
        let dw_local_player = memory.read_mut::<LocalPlayer>(0xDA244C);
        println!("health = {:?}", (*dw_local_player).get_health());
        loop {
            if toy_arms::detect_keydown(VirtualKeyCode::VK_INSERT) {
                break;
            }
        }
    }

}