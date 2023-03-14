# Learning RUST

## Hello World program
File - main.rs
```rust
fn main() {
    println!("Hello, World!");
}
```

`main()` is the entrypoint in every rust program and using `fn` we can define a function.
`println!()` is a macro not a function. Will learn later about macros.

## Running rust program

### Using Rust compiler

```bash
rustc main.rs
```
This will generate a plaform dependent binary called `main` and we can execute that binary to run the program.

### Using Cargo
Cargo is a rust's build system and package manager.

Create a new rust project - 
```bash
cargo new hello-world
```

This is going to generate the following files - 
 - Cargo.toml - cargo configuration file containing project(name, version, edition - rust version) and dependencies info.
 - src/main.rs - the main rust file.

File - Cargo.toml
```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

[dependencies]
```

#### Cargo commands
To build the rust project
```bash
cargo build
```
This will generate the binary in target/debug/hello_world because the default build is a debug build.

To compile and run the rust project - this is smart if the source file is not changed, this will just run the binary else it will first compile the project then run the binary.
```bash
cargo run
```

To check whether the project will compile or not
```bash
cargo check
```

To build the project for release
```bash
cargo build --release
```
This will generate the binary in `target/release/hello_world` with optimizations.

To update the rust version to latest stable build
```bash
rustup
```