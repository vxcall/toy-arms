[![Crates.io](https://img.shields.io/crates/v/toy-arms?style=for-the-badge)](https://crates.io/crates/toy-arms)
[![Docs.rs](https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/toy-arms)

<div align="center">

# :crossed_swords: toy-arms
<img src="https://user-images.githubusercontent.com/33578715/155048461-cb5cdd3f-6d59-4558-b3be-ce8b78144953.png" />

Huge thanks to my pal for this header [@suzuharuR](https://twitter.com/suzuharuR)

[Usage](#Usage) | [Examples](#fire-minimal-examples) | [Document](https://docs.rs/toy-arms)

</div>

# What's toy-arms?
This repository is a library that enables game hack development in Rust. However, it is more than just a demonstration of the possibilities of Rust rather than anything special. It is probably interesting for those who only have experience developing in C++.

# :pushpin: Table of contents

- [:two_hearts: support me](#two_hearts-support-me)
- [:fire: Get started](#fire-get-started)
  - [step1](#step1)
  - [step2](#step2)
- [:scroll: Practical Examples](#scroll-practical-examples)
  - [internal](#internal)
    - [simplest dll](#simplest-dll-internal)
    - [auto shoot](#auto-shoot-internal)
    - [get localplayer health](#get-localplayer-health-internal)
    - [pattern scang](#pattern-scan-internal)
  - [external](#external)
    - [auto shoot](#auto-shoot-external)
    - [get localpalyer health](#get-localplayer-health-external)
    - [pattern scan](#pattern-scan-external)
- [:card_file_box: Other examples?](#card_file_box-other-examples)
- [:herb: API info](#herb-api-info)

# :fire: Get started

But before actually test the example, I'll show you some preparation steps you're supposed to know.

## step1
Firstly, include `toy-arms` in your dependencies' table in `Cargo.toml`.

As of now toy-arms has 2 features which are `internal` and `external`.
`internal` feature flag is on by default so you have to specify `external` when you wanna use it.

**for internal use:**
```toml
[dependencies]
toy-arms = {git = "version = "https://github.com/vxcall/toy-arms"}

# This annotation below is to tell the compiler to compile this into dll. MUST.
[lib]
crate-type = ["cdylib"]
```

**for external use:**
```toml
[dependencies]
toy-arms = {git = "https://github.com/vxcall/toy-arms", features = ["external"]}
```

## step2

Secondly, sicne most of those tests are targeting the game "csgo.exe(x86)", you may have to build the code in x86 architecture depending on the example.
You can either specify in `.cargo/config.toml` as following:
```toml
[build]
target = "i686-pc-windows-msvc"
```

Or put `--target i686-pc-windows-msvc` flag everytime when you build the code.

If you don't have toolchain for 32bit msvc, do following
```shell
 rustup target add i686-pc-windows-msvc 
```

# :scroll: Practical Examples

In this section I'll showcase you various examples for different situations with internal and external features. Find one fits your purpose.

Be informed that all these examples are happened to target **CSGO:counter strike global offencive**
, so be sure to get it and test with it.


## internal

Welcome to the examples of internal hack. 
A dll file will be generated by build these examples, inject it with whatever dll injector you possess.

### simplest dll (internal)

With this crate, making the injectable dll which is the smallest possible is simple as this:

`cargo b --example in_simplest_dll --target i686-pc-windows-msvc`

```rust
/*
This is the demonstration of how to make the simplest hack with toy-arms.
*/
// A neat macro which defines entry point instead of you.
// Also, you dont have to alloc/free console by yourself, console will show up when u compile into debug build.

internal::create_entrypoint!(hack_main_thread);

// Main thread
fn hack_main_thread() {
    // YOUR STUNNING CODE'S SUPPOSED TO BE HERE;
    for i in 0..30000 {
        println!("using toy-arms {}", i);
    }
}
```

### auto shoot (internal)

This is the code that overwrites the value at `DW_FORCE_ATTACK` to 0x5 every loop in csgo.exe.
Note that you have to check if the address of `DW_FORCE_ATTACK` is up-to-date.

`cargo b --example in_auto_shoot --target i686-pc-windows-msvc`

```rust
/*
This is the demonstration of how to use internal analysis feature in toy-arms.
This code gets module handle and function address of the func called MessageBoxA as an example.
Then read the value called dwForceAttack and overwrite it to make player shoot.
The offset DW_FORCE_ATTACK works as of the day i wrote this but it might not be up to date in your case.
*/

use toy_arms::internal::cast;
use toy_arms::utils::detect_keydown;
use toy_arms::utils::keyboard::{detect_keypress, VirtualKeyCode};
use internal::common::get_module_handle;
use winapi::shared::minwindef::HMODULE;

internal::create_entrypoint!(hack_main_thread);

// This offset has to be up to date.
const DW_FORCE_ATTACK: usize = 0x320BDE8;

fn hack_main_thread() {
    let mut once = false;

    // Gets module handle
    let module_handle: HMODULE = get_module_handle("client.dll").unwrap();
    println!("module handle = {:?}", module_handle as usize);

    let shoot_flag = cast!(mut module_handle as usize + DW_FORCE_ATTACK, u8);

    loop {
        if !once {
            println!("Press INSERT to exit...");
            once = !once;
        }

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
```

### get localplayer health (internal)

While this code below will retrieve health value of LocalPlayer object in csgo.exe.
Note that you have to update the offset of `DW_LOCAL_PLAYER`.

`cargo b --example in_get_localplayer_health --target i686-pc-windows-msvc`

```rust
/*
This example is the demonstration of getting player health with toy-arms internal memory analysis feature.
Make sure that you inject this image to csgo.exe.
also, the offset of DW_LOCAL_PLAYER works as of the day i wrote this but it might not be up to date in your case.
*/
use internal::cast;
use internal::module::Module;
use internal::GameObject;
use toy_arms::derive::GameObject;
use utils::keyboard::VirtualKeyCode;

internal::create_entrypoint!(hack_main_thread);

// This macro provides from_raw() func that ensures the base address is not null.
#[derive(GameObject)]
struct LocalPlayer {
    pointer: *const usize, // Denote the base address of LocalPlayer to use it later in get_health() function.
}

impl LocalPlayer {
    unsafe fn get_health(&self) -> u16 {
        *cast!(self.pointer as usize + 0x100, u16)
    }
}

// This offset has to be up to date.
const DW_LOCAL_PLAYER: u32 = 0xDBF4BC;

fn hack_main_thread() {
    let module = Module::from_name("client.dll").unwrap();
    unsafe {
        //let dw_local_player = memory.read_mut::<LocalPlayer>(0xDA244C);
        loop {
            if let Some(i) = LocalPlayer::from_raw(module.read(DW_LOCAL_PLAYER as usize)) {
                println!("health = {:?}", (*i).get_health());
            };
            if toy_arms::utils::keyboard::detect_keypress(VirtualKeyCode::VK_INSERT) {
                break;
            }
        }
    }
}
```

### pattern scan (internal)

This is the pattern scanning example where the pattern is for dwForceAttack in csgo.

`cargo b --example in_pattern_scan --target i686-pc-windows-msvc`

```rust
/*
This is an example to demonstrate how to use powerful pattern scan feature in toy-arms.
Make sure you inject this image to csgo.exe.
The model pattern is for dwForceAttack.
*/

use internal::module::Module;
use toy_arms::utils::keyboard::{detect_keypress, VirtualKeyCode};
internal::create_entrypoint!(hack_main_thread);

const DW_FORCE_ATTACK_PATTERN: &str = "89 0D ? ? ? ? 8B 0D ? ? ? ? 8B F2 8B C1 83 CE 04";

fn hack_main_thread() {
    let mut once = false;

    let mut client = Module::from_name("client.dll").unwrap();

    match client.find_pattern(DW_FORCE_ATTACK_PATTERN) {
        Some(i) => println!("[+] *dwForceAttack address: 0x{:x}", i),
        None => println!("[-] Pattern not found"),
    }

    match client.pattern_scan(DW_FORCE_ATTACK_PATTERN, 2, 0) {
        Some(i) => println!("[+] dwForceAttack address: 0x{:x}", i),
        None => println!("[-] Offset not found"),
    }

    loop {
        if !once {
            println!("[+] Press INSERT to exit...");
            once = !once;
        }
        // To exit this hack loop when you input INSEERT KEY
        if detect_keypress(VirtualKeyCode::VK_INSERT) {
            break;
        }
    }
}
```

## external

On the other hand, following code is how tamper with memory externally is like.

### auto shoot (external)

This is the code that overwrites the value at `DW_FORCE_ATTACK` to 0x5 every loop in csgo.exe.
Note that you have to check if the address of `DW_FORCE_ATTACK` is up-to-date.

`cargo r --example ex_auto_shoot --features external --no-default-features`

```rust
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
```

### get localplayer health (external)

Retrieving local player health demo is present as below.
Quick tip: You don't wanna use usize as the pointer type cuz external program itself is going to be 64bit, 
therefore the buffer size will be 8bytes whereas the actual pointer is 4bytes. Use u32 or DWORD instead.

`cargo r --example ex_get_localplayer_health --features external --no-default-features`

```rust
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
```

### pattern scan (external)

This is the pattern scanning example where the pattern is for dwForceAttack in csgo.

`cargo r --example ex_pattern_scan --features external --no-default-features`

```rust
use toy_arms::external::process::Process;
use toy_arms::utils::keyboard::VirtualKeyCode;

const DW_FORCE_ATTACK_PATTERN: &str = "89 0D ? ? ? ? 8B 0D ? ? ? ? 8B F2 8B C1 83 CE 04";

fn main() {
    let mut once = false;

    // Getting process information
    let process = Process::from_process_name("csgo.exe").unwrap();

    // You can get module information by using get_client
    let mut client = process.get_module_info("client.dll").unwrap();

    let address = client.find_pattern(DW_FORCE_ATTACK_PATTERN);
    match address {
        Some(i) => println!("[+] found *dwForceAttack pattern at 0x{:x}", i),
        None => println!("[-] NOTHING FOUND"),
    }

    let offset = client.pattern_scan::<u32>(DW_FORCE_ATTACK_PATTERN, 2, 0);
    match offset {
        Some(i) => println!("[+] found dwForceAttack offset at 0x{:x}", i),
        None => println!("NOTHING FOUND"),
    }

    loop {
        if !once {
            println!("[+] Press INSERT to exit...");
            once = !once;
        }
        // Exit this loop by pressing INSERT
        if toy_arms::utils::detect_keydown!(VirtualKeyCode::VK_INSERT) {
            break;
        }
    }
}
```

# :card_file_box: Other examples?
Yes you have! Take a look at [examples directory](https://github.com/s3pt3mb3r/toy-arms/tree/master/examples), you'll see more examples!

However, you may need to update offsets which some examples contain with your own hands.

Refer to [hazedumper](https://github.com/frk1/hazedumper/blob/master/csgo.hpp) as always for latest offsets of CSGO.

To build examples in x86 arch:
```shell
cargo build --example EXAMPLE_NAME --target i686-pc-windows-msvc
```

# :herb: API info

### external `read()` func

```rs
fn read<T>(
    process_handle: &HANDLE,
    base_address: usize,
    size: usize,
    buffer: *mut T,
)
```

make sure to pass buffer like following:

```rs
let mut buffer: u32 = 0; // Declare with mut keyword
read::<u32>(
    &handle,
    base_address,
    size_of::<u32>(),
    &mut buffer as *mut u32, // Must be the form of &mut buffer as *mut T
    // These are equivalent to &buffer in C++.
);
```

### internal `read()` func and `cast!()` macro

in the example, both of them are used conditionally. They are present for different use.

`cast!()` is the basic dereference and u can always use this of course.

`read()` is the member method of Module struct. It allows u to just pass the offset of what u want from module base, and it adds them for you. `cast!()` is used under the hood.

# Acknowledge
[hazedumper-rs](https://github.com/frk1/hazedumper-rs) - referenced as role model of pattern scan
