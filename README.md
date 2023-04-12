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

To compile and run the rust project - this is smart if the source file is not changed, this will
just run the binary else it will first compile the project then run the binary.
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

# Chapter - 3: Common programming concepts

## Variables and mutability

Variables in rust are by default immutable.

```rust
fn main() {
    let x = 10;
    x = 16;     // error because of immutability
}
```

### Immutable keyword

We can make a variable immutable with the help of `mut` keyword. 
With this variable can hold another value of same type.

```rust
fn main() {
    let mut x = 10;
    x = 10.09;  // invalid
    x = 16;     // valid because changing the value to same type
}
```

### Constants

We can make a variable constant with the help of `const` keyword.
Rust's naming convention for constants is to use *all uppercase with
underscores between words*.

```rust
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
```

### Shadowing
We can shadow a variable by using the same variable's name and repeating the use
of the `let` keyword.

```rust
fn main() {
    let x = 5;
    let x = x + 1;              // x = 5 + 1
    {
        let x = x * 2;          // x = 6 * 2
    }
    let x = "Hello, World!";    // valid
}
```

## Data types

Rust has two data type subsets: **Scalar** and **Compound**.
We must specify the data type of variable while declaring a variable.

```rust
fn main() {
    let x: u32 = 10;
}
```

### Scalar types

A scalar type represents a single value. Rust has four primary scalar types: 
**integers, floating-point numbers, boolean, and character**.
> Signed numbers are stored using two's complement representation.

#### Integer types:
Integer types default to `i32`.

| **Length** | **Signed** | **Unsigned** |
|:----------:|:----------:|:------------:|
|    8-bit   |     i8     |      u8      |
|   16-bit   |     i16    |      u16     |
|   32-bit   |     i32    |      u32     |
|   64-bit   |     i64    |      u64     |
|   128-bit  |    i128    |     u128     |
|    arch    |    isize   |     usize    |

#### Integer literals:
| **Number literals** | **Example** |
|:-------------------:|:-----------:|
|       Decimal       |    65_536   |
|         Hex         |     0xff    |
|        Octal        |     0o77    |
|        Binary       | 0b1111_0000 |
|    Byte (u8 only)   |     b'A'    |

#### Floating-point types:
Rust's floating-point types are `f32` and `f64`, which are 32 bits and 64 bits in size.
The default type is `f64`. All floating-point types are signed.

`f32` type is a single-precision float and `f64` has double precision.
```rust
fn main() {
    let x = 2.0;        // f64
    let y: f32 = 3.0;   // f32
}
```

#### Numeric operations
```rust
fn main() {
    let sum = 10 + 5;
    let difference = 10.98 - 3.4;
    let product = 4 * 1.4;
    let quotient = (5 as f32) / 3.0;
    let truncated = 22 / 7;
    let remainder = 22 % 7;
}
```

#### Boolean:
Boolean is of one byte in size.
```rust
fn main() {
    let t = true;
    let f: bool = false;
}
```

#### Character types:
Char is of four bytes in size and represents a Unicode scalar value (similar to Java).
```rust
fn main() {
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😘';
}
```

### Compound types:
Compound types can group multiple values into one type. Rust has two primitive compound types:
**tuples and arrays**.

#### Tuple:
Tuple is used to group together a number of values with different types into one compound type.
Tuples have a fixed length: once declared, they cannot grow or shink in size.
It is created using a comma-seperated list of values inside paranthesis.

**Access an element of tuple:** we can access a tuple element using a period (`.`) followed by the
index of the value we want to access.

```rust
fn main() {
    let tuple: (i32, f64, u8) = (500, 6.4, 1);      // creating a tuple
    let (x, y, z) = tuple;                          // destructuring a tuple
    let a = tuple.0;                                // a = 500
    println!("{:?}", tuple);
}
```

#### Array:
It stores the value of same type. Arrays in Rust have a fixed length.
Arrays stores data on the stack rather than the heap. An array isn't as flexible
as the `vector` type.

We cannot change the value at a perticular index because by default it is immutable.


Accessing an out-of-bound index will result in panic (Exception).

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];                    // declaration + initialization
    let a: [i32; 5] = [1, 2, 3, 4, 5];          // we must declare an array with the type and length [type; length]
    let a = [3; 5];                             // a = [3, 3, 3, 3, 3]
    println!("Array: {:?}", a);                 // Array: [3, 3, 3, 3, 3]

    let mut a = [3; 2];
    a[0] = 1;
    a[1] = 2;

    println!("a[0]: {}, a[1]: {}", a[0], a[1]); // a[0]: 1, a[1]: 2

    let a: [i32; 5] = [0; 5];
    println!("Array: {:?}", a);                 // Array: [0, 0, 0, 0, 0]
}
```