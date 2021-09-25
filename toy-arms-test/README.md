# Command to build specifically this package.

Cargo doesn't read `.cargo/config` file when you build from root directory, so you have to build with either

```shell
cd toy-arms-test
cargo build
```

or

```shell
cargo build -p toy-arms-test --target i686-pc-windows-msvc
```