![Crates.io](https://img.shields.io/crates/v/toy-arms?style=for-the-badge) 
# :alembic: toy-arms
Windows game hack helper utilities in rust.

This crate has some useful macros, functions and traits.

# :fire: How to use this crate?
## internal
With this crate, making dll is simple as this:

```rust
// A neat macro which defines entry point instead of you.
// Also, you dont have to alloc/free console by yourself, console will show up only when debug compile.
toy_arms::create_entrypoint!(hack_main_thread);

// Main thread
fn hack_main_thread() {
    // YOUR STUNNING CODE'S SUPPOSED TO BE HERE;
    for i in 0..30000 {
        println!("using toy-arms {}", i);
    }
}
```

## external
On the other hand, following code is how tamper with memory externally is like.

Since this one shows some example usage of the crate's features, it looks a bit fancier.

```rust
use toy_arms::{MemoryEx, VirtualKeyCode};

fn main() {
    // This const has to be up to date.
    const DW_FORCE_ATTACK: usize = 0x31EDB20;
    // Getting process information
    let memex = MemoryEx::from_process_name("csgo.exe");
    println!("process id = {}, \nprocess handle = {:?}", memex.process_id, memex.process_handle);

    // You can get module information by using get_module_info
    let module_info = memex.get_module_info("client.dll").unwrap();
    println!("{}", module_info.module_name);

    // read fetches the value at where the address is pointing.
    // U have to specify the type of the value with turbofish
    println!("{:?}", memex.read::<u32>(memex.get_module_base("client.dll").unwrap() + DW_FORCE_ATTACK as usize).unwrap());

    loop {

        // write helps you tamper with the value.
        memex.write::<u32>(memex.get_module_base("client.dll").unwrap() + DW_FORCE_ATTACK as usize, &mut 0x5).unwrap();

        // Exit this loop by pressing INSERT
        if toy_arms::detect_keydown(VirtualKeyCode::VK_INSERT) {
            break;
        }
    }
}
```

# :globe_with_meridians: Other examples?
Take a look at examples directory, you'll see more examples!
To run the example:
```shell
cargo build --example EXAMPLE_NAME
```