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

# Chapter - 2: Guessing game

### Take input
```rust
use std::io;

fn main() {
    let mut input = String::new(); // empty string
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to take input");

    println!("Input value: {}", input);
}
```

### Convert String to another type

```rust
fn main() {
    let number = "10";
    let number = number.trim().parse().expect("Fail to parse the number");
}
```

### Compare two numbers
Comparison in rust returns Ordering enum so we can use match to handle that.
```rust
use std::cmp::Ordering;

fn main() {
    let number = 10;
    let another_number = 100;
    match number.cmp(&another_number) {
        Ordering::Less => println!("Smaller number"),
        Ordering::Equal => println!("Equal numbers"),
        Ordering::Greater => println!("Greater number")
    };
}
```

### Generate a random number
First add the `rand` dependency
```bash
cargo add rand
```

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100); // generating a random number in the range [1, 100]
    println!("Random number: {}", secret_number);
}
```

### Add loop
Currently, it will only take the input once but we want to take input till correct number is guessed.

```rust
loop {
    ... generate random number, take input, convert string to integer

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => {
            println!("You guessed it right!");
            break;
        }
        Ordering::Greater => println!("Too big!")
    }
}
```

### Handle invalid input
If we try to parse a string into an integer with the non-digit value then it will throw an error.
Since `parse()` returns the `Result` enum using which we can handle the error.

```rust
fn main() {
    let number = "10c";
    number: u32 = match number.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type a number!");
            continue;
        } // _ denotes the catchall value
    }
}
```