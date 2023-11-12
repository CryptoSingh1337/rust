# Learning RUST

## Chapter - 1: Hello World program
File - main.rs
```rust
fn main() {
    println!("Hello, World!");
}
```

`main()` is the entrypoint in every rust program and using `fn` we can define a function.
`println!()` is a macro not a function. Will learn later about macros.

### Running rust program

#### Using Rust compiler

```bash
rustc main.rs
```
This will generate a plaform dependent binary called `main` and we can execute that binary to run the program.

#### Using Cargo
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

##### Cargo commands
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
rustup update
```

## Chapter - 2: Common programming concepts

### 2.1 Variables and mutability

Variables in rust are by default immutable.

```rust
fn main() {
    let x = 10;
    x = 16;     // error because of immutability
}
```

#### Immutable keyword

We can make a variable immutable with the help of `mut` keyword. 
With this variable can hold another value of same type.

```rust
fn main() {
    let mut x = 10;
    x = 10.09;  // invalid
    x = 16;     // valid because changing the value to same type
}
```

#### Constants

We can make a variable constant with the help of `const` keyword.
Rust's naming convention for constants is to use *all uppercase with
underscores between words*.

```rust
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
```

#### Shadowing
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

### 2.2 Data types

Rust has two data type subsets: **Scalar** and **Compound**.
We must specify the data type of variable while declaring a variable.

```rust
fn main() {
    let x: u32 = 10;
}
```

#### Scalar types

A scalar type represents a single value. Rust has four primary scalar types: 
**integers, floating-point numbers, boolean, and character**.
> Signed numbers are stored using two's complement representation.

##### Integer types
Integer types default to `i32`.

| **Length** | **Signed** | **Unsigned** |
|:----------:|:----------:|:------------:|
|    8-bit   |     i8     |      u8      |
|   16-bit   |     i16    |      u16     |
|   32-bit   |     i32    |      u32     |
|   64-bit   |     i64    |      u64     |
|   128-bit  |    i128    |     u128     |
|    arch    |    isize   |     usize    |

##### Integer literals
| **Number literals** | **Example** |
|:-------------------:|:-----------:|
|       Decimal       |    65_536   |
|         Hex         |     0xff    |
|        Octal        |     0o77    |
|        Binary       | 0b1111_0000 |
|    Byte (u8 only)   |     b'A'    |

##### Floating-point types
Rust's floating-point types are `f32` and `f64`, which are 32 bits and 64 bits in size.
The default type is `f64`. All floating-point types are signed.

`f32` type is a single-precision float and `f64` has double precision.
```rust
fn main() {
    let x = 2.0;        // f64
    let y: f32 = 3.0;   // f32
}
```

##### Numeric operations
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

> Rust does not support ++ and -- operators

##### Boolean
Boolean is of one byte in size.
```rust
fn main() {
    let t = true;
    let f: bool = false;
}
```

##### Character types
Char is of four bytes in size and represents a Unicode scalar value (similar to Java).
```rust
fn main() {
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😘';
}
```

#### Compound types
Compound types can group multiple values into one type. Rust has two primitive compound types:
**tuples and arrays**.

##### Tuple
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
> Note: it is recommended to use tuple for only 1-3 values not more than that it becomes quite confusing
> if we have more than 3 variable to access.

##### Array
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

### 2.3 Functions

In rust, functions are created using the keyword `fn`. `main` function is the entry point of many programs.
Rust code uses *snake_case* as the conventional style for function and variable names.

Functions must have type specified with the parameters.

```rust
fn print_labeled_measurement(value: i32, unit_label: char) -> () {
    println!("The measurement is: {}{}", value, unit_label);
}

fn main() {
    print_labeled_measurement(10, 'M');
}
```

#### The println macro

In rust, `println!` is a macro as in the end there is `!` which indicates its a macro not a function. It is use to print the 
output in the terminal. We can use this similar to the `printf` which can take dynamic values as an argument.
- Macros use exclamation point to call/invoke.
- Generate additional Rust code.
- Data can be 

Example:
```rust
println!("Number is: {:?}", input_number);  // Debug print
println!("Number is: {}", input_number);    // This is also valid
println!("Number is: {input_number:?}");    // Print the variable directly
println!("Number is: {input_number}");      // This is also valid but not the debug print
```
> Note: In println macro, :? denotes that we want to print this in debug mode only.

#### Statements and Expressions

Function bodies are made up of a series of statements optionally ending in an expression.
- **Statements** are instructions that perform some action and do not return a value.
- **Expressions** evaluate to a resultant value.

> `let` keyword is a statement, thus it does not return a value. So we cannot do let x = (let y = 6);

A new scope block created with curly brackets is an expression.

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}
```
This is the expression in above example:
```rust
{
    let x = 3;
    x + 1
}
```

#### Functions with return values

Functions can return values to the code that calls them. We must declare the type after an arrow (`->`).
In Rust, the return value of the function is same as the value of the final expression in the block of the body of a function.

Although we can use `return` keyword to return from the function, but most functions return the last expression implicitly.

> Function declaration order does not matter in Rust but it must be in the scope.

```rust
fn five() -> u8 {
    5
}

fn main() {
    let x = five();
    println!("X: {}", x);
}
```
In the above example, function implicitly return the value `5` because as mentioned above **last expression without semi-colon** return
the value implicitly.

```rust
fn plus_one(x: u8) -> u8 {
    x + 1;
}

fn main() {
    let x = plus_one(10);   // invalid because the last line becomes the statement instead of an expression
}
```

### 2.4 Comments

Rust has two kinds of comments: *line comments* and *doc comments*.

Line comments:
```rust
// This function increments the value of the argument by 1 and returns that.
fn plus_one(x: u8) -> u8 {
    x + 1
}
```

Doc comments: supports markdown notation inside

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}
```

### 2.5 Control flow

#### If expression
```rust
fn main() {
    if condition {
        statements
    } else {
        statements
    }
}
```

> In Rust, it is mandatory to create the if block with curly brackets otherwise compiler will give error

If the condition isn't a `bool`, we'll get and error.
```rust
fn main() {
    let n = 5;
    if n {
        println!("Number was five");
    }
}
```
Rust expected a `bool` but got an integer. Unlike other languages like JavaScript, Rust will not
automatically try to convert non-Boolean types to a Boolean.

#### Multiple conditions with `else if`
```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        statements
    } else if number % 3 == 0 {
        statements
    } else if number % 2 == 0 {
        statements
    } else {
        statements
    }
}
```

#### Using `if` in a `let` statement
```rust
fn main() {
    let number = if condition { 5 } else { 6 };
    let number = if condition { 5 } else { "x" };   // this will result in error because it is creating ambiguity
}
```

#### Loops

##### `loop` keyword

This can be use as infinite loop also.
```rust
fn main() {
    loop {

        if condition {
            break;
        }
    }
}
```

##### Returning values from loop
```rust
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);   // The result is 20
}
```

##### `While` loop
```rust
fn main() {
    let mut idx = 0;
    let a = [1, 2, 3, 4, 5];
    while idx != a.len() {
        println!("a[{}]: {}", idx, a[idx]);
        idx += 1;
    }
}
```

##### `For` loop
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    for element in a {
        println!("Number: {}", element);
    }

    for number in (1..=5).rev() {
        println!("Number: {}", number);
    }
}
```

#### `Match` keyword

`match` allows us to compare a value against a series of patterns and then execute code based on which pattern matches simiar
to `switch` case. It is also use for flow control. It is exhaustive i.e., all options/cases must be considered. It works on
expression not on the statements. We can handle any other case using `_`.

**`match` v/s `else-if`**:
- `match` will be checked by the compiler, if a new possibility is added, we will be notified.
- `else-if` is not checked by the compiler, if a new possibility is added then code may encounter with a new bug.

```rust
fn main() {
    let bool_value = false;
    match bool_value {
        true => println!("Its true"),
        false => println!("Its false")
    }
}
```

```rust
fn main() {
    let x = 4;
    match x {
        1 => println!("It's 1"),
        2 => println!("It's 2"),
        4 => println!("It's 4"),
        _ => println!("It's something else")    // Default case
    }
}
```

## Chapter - 3: Enums

1. Data that can be one of multiple different possibility, each possibility is called a "variant".
2. Provides information about your program to the compiler, more robust program.
3. Enums can be only one variant at a time.
4. More robust programs when paired with match.

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn which_way(go: Direction) {
    match go {
        Direction::Up => "Up",
        Direction::Down => "Down",
        Direction::Left => "Left",
        Direction::Right => "Right"
    }
}
```

We can use struct but the problem will be how we can pass that in function because each struct will be different.
We can also pass some data with the enum type while creating using the constructor.

```rust
enum Direction {
    Up(String),
    Down(String),
    Left(String),
    Right(String)
}
```

## Chapter - 4: Struct

1. A type that contains multiple pieces of data.
    i. All or nothing - cannot have some pieces of data and not others.
2. Each piece of data is called a 'field'.
3. Makes working with data easier.
    i. Similar data can be grouped together.


```rust
struct Box {
    width: u32,
    height: u32,
    depth: u32
}

fn main() {
    let cube = Box {
        width: 3,
        height: 3,
        depth: 3
    };

    println!("The cube width: {cube.width:?}, height: {cube.height:?}, depth: {cube.depth:?}");
}
```

## Guessing game

### Take input
```rust
use std::io;

fn main() {
    let mut input = String::new(); // empty string
    io::stdin()
        .read_line(&mut input)
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

A `match` expression is made up of *arms*. An arm consists of a pattern to match against, and the code that
should be run if the value given to `match` fits that arm's pattern.

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