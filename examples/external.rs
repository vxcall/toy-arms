use toy_arms::MemoryEx;

fn main() {
    let memex = MemoryEx::from_process_name("csgo.exe");
    println!("process id = {}", memex.process_id);
    println!("process handle = {:?}", memex.process_handle);
    let module_info = memex.get_module_info("client.dll").unwrap();
    println!("{:?}", module_info.modBaseAddr);
    loop {

    }
}