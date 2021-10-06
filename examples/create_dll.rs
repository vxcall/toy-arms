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