use external::error::{ReadWriteMemoryFailedDetail, TAExternalError};
use std::mem::size_of;
use toy_arms::external::error::TAExternalError::ReadMemoryFailed;
use toy_arms::external::module::Module;
use toy_arms::external::process::Process;
use toy_arms::external::read;

const DW_LOCAL_PLAYER: u32 = 0xDBF4BC;

fn main() {
    let csgo: Process;
    match Process::from_process_name("csgo.exe") {
        Ok(p) => csgo = p,
        Err(e) => {
            println!("{}", e);
            return;
        }
    }
    let client: Module;
    match csgo.get_module_info("client.dll") {
        Ok(m) => client = m,
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    println!("[+] module_base: {:x}", client.base_address);
    println!(
        "[+] localplayer pointer pointer: 0x{:x}",
        client.base_address + DW_LOCAL_PLAYER as usize
    );

    let mut localplayer: u32 = 0;
    let ok = read::<u32>(
        &csgo.handle,
        client.base_address + DW_LOCAL_PLAYER as usize,
        size_of::<u32>(),
        &mut localplayer as *mut u32,
    );

    match ok {
        Ok(_ok) => {
            println!("[+] localplayer pointer: 0x{:x}", localplayer);
            let mut health: u16 = 0;
            // 0x100 is the offset of the health in player entity class.
            let ok2 = read::<u16>(
                &csgo.handle,
                localplayer as usize + 0x100,
                size_of::<u16>(),
                &mut health as *mut u16,
            );
            match ok2 {
                // This is what we wanted.
                Ok(h) => println!("[+] localplayer's health: {}", health),
                Err(ReadMemoryFailed(e)) => println!("{}", e),
                Err(_) => println!("[-] some error"),
            }
        }
        Err(e) => match e {
            TAExternalError::ReadMemoryFailed(ReadWriteMemoryFailedDetail::ErrorPartialCopy) => {
                println!("Partial Copy. Probably the address is protected")
            }
            TAExternalError::ReadMemoryFailed(ReadWriteMemoryFailedDetail::ErrorInvalidAddress) => {
                println!("Invalid Address")
            }
            TAExternalError::ReadMemoryFailed(ReadWriteMemoryFailedDetail::ErrorInvalidHandle) => {
                println!("Invalid Handle")
            }
            TAExternalError::ReadMemoryFailed(ReadWriteMemoryFailedDetail::UnknownError {
                error_code,
            }) => println!("Unknown Error: {}", error_code),
            _ => println!(
                "[-] error: {}\n[-] Maybe non-updated offset are the reason. update it yourself.",
                e
            ),
        },
    }
}
