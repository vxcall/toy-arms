![Crates.io](https://img.shields.io/crates/v/toy-arms?style=for-the-badge) 
# :alembic: toy-arms
Windows game hack helper utilities in rust.

This crate has some useful macros, functions and traits.

# :fire: How to use this crate?
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

# :globe_with_meridians: Other examples?
Take a look at examples directory, you'll see more examples!
To run the example:
```shell
cargo build --example EXAMPLE_NAME
```