use toy_arms::external::{Module, Process, read};
use toy_arms::external::error::TAExternalError::ReadMemoryFailed;

const DW_LOCAL_PLAYER: u32 = 0xDB35DC;

fn main() {
    let csgo: Process;
    match Process::from_process_name("csgo.exe") {
        Ok(p) => csgo = p,
        Err(e) => {
            println!("{}", e);
            return;
        },
    }
    let client: Module;
    match csgo.get_module_info("client.dll") {
        Ok(m) => client = m,
        Err(e) => {
            println!("{}", e);
            return;
        },
    }

    println!("module_base: {:x}", client.module_base_address);
    println!("localplayer pointer: 0x{:x}", client.module_base_address + DW_LOCAL_PLAYER as usize);
    let localplayer = read::<u32>(csgo.process_handle, client.module_base_address + DW_LOCAL_PLAYER as usize);

    match localplayer {
        Ok(l) => {
            println!("localplayer address: 0x{:x}", l);
            // 0x100 is the offset of the health in player entity class.
            let health = read::<u16>(csgo.process_handle, l as usize + 0x100);
            match health {
                // This is what we wanted.
                Ok(h) => println!("localplayer's health: {}", h),
                Err(ReadMemoryFailed(e)) => println!("{}", e),
                Err(_) => println!("some error"),
            }
        },
        Err(e) => println!("error: {}", e),
    }
}