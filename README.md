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
> Note: We can use `rustup doc` to open the rust documentation

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

##### `While let` loop
```rust
let mut data = Some(3);
while let Some(i) = data {
    println!("loop");       // Only print only 1 time
    data = None
}

let numbers = vec![1, 2, 3];
let mut number_iter = number.iter();
while let Some(num) = number_iter.next() {
    println!("Number: {num}");
}
```
>**Note:** This is useful when we are working with iterators.

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
        other => println!("Other number: {:?}", other)    // Default case
    }
}
```

>**Note:** We can use if let block also if we only care about the one variation

```rust
fn main() {
    let maybe_user = Some("John");
    match maybe_user {
        Some(user) => println!("User: {user}"),
        None => println!("No user")
    }

    if let Some(user) = maybe_user {
        println!("User: {user}")
    }
}
```
**Advance Match**

We can match `enum` and `struct` based on some values they are having.

```rust
enum Discount {
    Percent(f32),
    Flat(i32)
}

struct Ticket {
    event: String,
    price: i32
}

fn main() {
    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("flat 2"),
        Discount::Flat(amount) => println!("flat discount of {:?}", amount),    // This implicity returns nothing
        _ => ()                                                                 // Return nothing explicitly
    }

    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50.0
    };

    match concert { 
        Ticket { price: 50, event } => println!("event @ 50: {:?}", event),
        Ticket { price, .. } => println!("price: {:?}", price)
    }
}
```
> Note: We can use `..` to ignore other fields while matching against `struct`

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

enum PromoDiscount {
    NewUser,
    Holiday(String)
}

enum Discount {
    Percent(f64),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String)
}
```
1. Enum variants can optionally contain data
    i. The data can be another enum as well
2. Can mix plain identifiers and data-containing varaints within the same enum
3. More than one piece of data can be associated with a varaint

## Chapter - 4: Struct

1. A type that contains multiple pieces of data.
    i. All or nothing - cannot have some pieces of data and not others.
2. Each piece of data is called a 'field'.
3. Makes working with data easier.
    i. Similar data can be grouped together.

> Note: Struct can only have owned items not the borrowed items

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

## Chapter - 5: Ownership

Rust uses the concept of the ownership model which states that if we create a variable in a function that function is the owner of that variable.
Once the function executed completely then that variable will be deleted.

```rust
enum Light {
    Bright,
    Dull
}

fn display_light(light: Light) {
    match light {
        Light::Bright => println!("Bright light"),
        Light::Dull => println!("Dull light")
    }
}

fn main() {
    let dull = Light::Dull;
    display_light(dull);    // till this point program will get compile
    display_light(dull);    // this is the error
}
```
In the above example, `main` function is the owner of the `dull` variable and when it call `display_light` function the ownership is transfered (or moved) to `display_light` function and when `display_light` executed completely then light variable got deleted. So when the pointer returns back to `main` function the `dull` variable already got deleted.

To fix this, we should use the borrowing concept which is similar to passing the reference not the ownership.

```rust
fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("Bright light"),
        Light::Dull => println!("Dull light")
    }
}

fn main() {
    let dull = Light::Dull;
    display_light(&dull);
    display_light(&dull);
}
```
This will work perfectly fine because the owner of the `dull` is currently the `main` function only and we are only passing the reference of the `dull` to the `display_light` function which is perfectly fine. The `main` function will delete this `dull` variable.

- Memory must be managed in some way to prevent leaks.
- Rust uses `ownership` to accomplish memory management.
    - The *owner* of data must clean up the memory.
    - This occurs automatically at the end of the scope.
- Default behavior is to 'move' memory to a new owner.
    - Use an  ampersand `&` to allow code to `borrow` memory.

## Chapter - 6: Impl keyword

`Impl` allows us to implement functionality on enumerations and structures. In implementation block we can also use reference to self with the `&self`.
This is similar to creating a method in a java class.

#### `&self` v/s `Self`

`&self` indicates the we already have the object of a particular type but `Self` indicates that we don't have object and we are creating a new one.

```rust
struct Temperature {
    degrees_c: f32
}

impl Temperature {
    // fn show_temp(temp: Temperature) {
    //     println!("{:?} degrees C", temp.degrees_c);
    // }

    fn freezing() -> Self {
        Self { degrees_c: -1.2 }
    }

    fn show_temp(&self) {
        println!("{:?} degrees C", self.degrees_c);
    }
}

fn main() {
    let hot = Temperature { degrees_c: 40.2 };
    // Temperature::show_temp(hot);
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();
}
```

#### How to create a constructor similar to new keyword in java

We can use this same functionality but the function name will be new which will take the arguments as input.

```rust
struct Car {
    weight: f32
}

impl Car {
    
    fn new(weight: f32) -> Self {
        Car { weight }
    }
}

fn main() {
    let car = Car::new(10.9);
}

```

## Chapter - 7: Vector

`Vector` is similar to `List`. It can only stores homogenous elements. We can create a vector using `vec!` macro or by using the `Vec::new()` function.

1. Vectors contain multiple pieces of similar data.
2. Data can be added or removed.
3. The `vec!` macro can be used to make vectors.
4. Use `for..in` to iterate through items of a vector.

```rust
fn main() {
    let numbers = vec![10, 20, 30, 40];

    for number in &numbers {
        match number {
            30 => println!("Number = thirty"),
            _ => println!("Number = {:?}", number),
        }
    }
    println!("Length of the vector = {:?}", numbers.len());
}
```
> Note: If we are iteratoring vector using for loop so vector ownership got transfer to iterator and we are trying to access vector to print the length so that vector got deleted.

## Chapter - 8: String

#### `String` v/s `&str`
- Two commonly used types of strings
    - `String` - owned
    - `&str` - borrowed String slice
- Must use an owned String to store in a `struct`
- Use `&str` when passing to a function
- Strings are by default borrowed
- Use `to_owned()` or `String::from()` to create an owned copy of a string slice
- Use an owned String when storing in a struct

```rust
fn print_it(data: &str) {
    println!("{:?}", data);
}

fn main() {
    print_it("a string slice");
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another");
    print_it(&owned_string);
    print_it(&another_string);
}
```

## Chapter - 9: Derive

It is use to derive the functionality into an `enum` or `struct`. It is a special type of macro which is use to derive functionality.
We can derive debug, clone, copy functionality.

**Syntax:**
```rust
#[derive(Debug, Clone, Copy)]
```

- `Debug` - it is used when we want to print the `enum` or `struct` using print macro with debug output.

```rust
#[derive(Debug)]
enum Position {
    Director,
    Manager,
    Supervisor,
    Worker
}

#[derive(Debug)]
struct Employee {
    position: Position,
    work_hours: u8
}

fn main() {
    let employee = Employee { position: Position::Worker, work_hours: 8 };
    println!("{:?}", employee.position);   // Error, if we are not using derive
    println!("{:?}", employee);
}
```
> Note: All fields of struct must also derive the functionality which struct is deriving.

- `Clone` & `Copy` - it is used to make a copy of the data which is stored in function or struct. Ownership is not going to transfer when we are passing the values to the struct or function, a copy is made is instead.

```rust
#[derive(Debug, Clone, Copy)]
struct Employee {
    name: String
}

fn print(employee: Employee) {
    println!("{:?}", employee);
}

fn main() {
    let employee = Employee { name: "Saransh".to_owned() };

    print(employee);    // Ownership is not move to print function because it is using Clone and Copy trait
    print(employee);
}
```
> Note: When print function is called, a copy of employee is created then it is passed to the print function. Remember only use Clone and Copy derive when the size of the struct is small i.e., with 2-3 fields.

## Chapter - 10: Option

**Definition:**
```rust
enum Option<T> {
    Some(T),
    None
}
```

`Option` is similar to `Optional` class in Java. It is used when their is the use case of `null` because rust doesn't have the concept of `null` values.

1. `Option` represents either some data or nothing.
    i. Some (variable_name) - Data is available
    ii. None - No data is available
2. Useful when needing to work with optional data.
3. Use `Option<type>` to declare an optional type.

**Example - 1:**
```rust
struct Customer {
    age: Option<i32>,
    email: String
}

let mark = Customer {
    age: Some(22), email: "mark@example.com".to_owned()
};

let becky = Customer {
    age: None, email: "becky@example.com".to_owned()
};

match becky.age {
    Some(age) => println!("customer is {:?} years old", age),
    None => println!("customer age not provided")
}
```

**Example - 2:**
```rust
struct GroceryItem {
    name: String,
    qty: i32
}

fn find_quantity(name: &str) -> Option<i32> {
    let groceries = vec![
        GroceryItem { name: "bananas".to_owned(), qty: 4 },
        GroceryItem { name: "bananas".to_owned(), qty: 12 },
        GroceryItem { name: "bananas".to_owned(), qty: 1 }
    ];

    for item in groceries {
        if item.name == name {
            return Some(item.qty);
        }
    }
    None
}
```

## Chapter - 11: Result

1. A data type that contains one of two types of data:
    - "Successful" data
    - "Error" data
2. Used in scenarios where an action needs to be taken, but has the possibility of failure
    - Copying a file
    - Connecting to a website

**Definition:**
```rust
enum Result<T, E> {
    Ok(T),
    Err(E)
}
```

**Example - 1:**
```rust
fn get_sound(name: &str) -> Result<SoundData, String> {
    if name == "alert" {
        Ok(SoundData::new("alert"))
    } else {
        Err("Unable to find sound data".to_owned())
    }
}

let sound = get_sound("alert");
match sound {
    Ok(_) => println!("Eound data located"),
    Err(e) => println!("Error: {:?}", e)
}
```

**Example - 2:**
```rust
#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Menu choice not found".to_owned())
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("Choice: {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String> {
    let choice: MenuChoice = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}

fn main() {
    pick_choice("start");
}
```
> Note: `?` denotes that we are not care about the `Err` Result so rust compiler automatically decompile that to `Ok`. If there is any error then it will directly result as `Err`.

## Chapter - 12: HashMap

**Syntax:**
```rust
let mut people = HashMap::new();
people.insert("Susan", 21);
people.insert("Ed", 13);
people.insert("William", 17);
people.remove("Susan");

// Retrieve data using Match expression
match people.get("Ed") => {
    Some(age) => println!("Age = {:?}", age),
    None => println!("Not found")
}

// Iterate the hashmap

for (name, age) in people.iter() {
    println!("Name: {:?}, age: {:?}", name, age);
}

for name in people.keys() {
    println!("Name: {:?}", name);
}

for age in people.values() {
    println!("Age: {:?}", age);
}
```

## Chapter - 13: Combinator

#### Closure

`Closures` are similar to function but they are defined inside another function. These are similar to lambda functions in `Java`. We can pass closure as a function parameter.

**Syntax:**
We use pipes `||` to define a closure.

```rust
fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let addition = add_fn(1, 1);

    let add = |a: i32, b: i32| -> i32 {     // This closure is same as function it has name, type etc.
        a + b
    }
    let addition = add(1, 1);

    let add = |a, b| a + b;                 // This is also a closure but with in concise manner
    let addition = add(1, 1);               // as rust compiler implicitly determine the types of parameters and the return type
}
```

#### Map combinator

Map combinator are use to map an `Option` type, similar to `map` intermediate stream operator.
It is an implementation for `Option` enum.

**Syntax:**
```rust
fn maybe_word() -> Option<String> {
    Some("Hello, world!")
}

fn main() {
    let word_length: Option<i32> = maybe_word()
        .map(|word| word.len())
        .map(|len| len * 2);
}
```

#### Option combinators
```rust
fn main() {
    let a = Some(1);
    let a_is_some = a.is_some();
    let a_is_none = a.is_none();
    let a_mapped = a.map(|num| num + 1);
    let a_filtered = a.filter(|num| num == &1);
    let a_or_else = a.or_else(|| Some(2));
    let unwrapped = a.unwrap_or_else(|| 0);
}
```

## Chapter - 14: Iterators

We can use iterators to iterate the any data structures.

**Syntax:**
```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let plus_one: Vec<_> = numbers
        .iter()
        .map(|num| num + 1)
        .collect();
}
```
>**Note:** We need to define type annotations because `iter()` & `collect()` are used for any data structure generically.

#### Methods:

- `map` - create a new value from the existing one.
- `filter` - filter out all the values which satisfies the predicate i.e., which return true.
- `find` - it finds the first element which satisfies the predicate condition it returns the `Option` value i.e., `Some` or `None`.
- `count` - count the no of items in the data structure
- `last` - return the last item its return type is `Option` i.e., `Some` or `None`.
- `min` - return the minimum value item.
- `max` - return the maximum value item.
- `take` - it take only the no of values which is provided as the argument.

## Chapter - 15: Ranges

It is used to generate the values between the given range.

**Syntax:**
```rust
fn main() {
    let range: Range<i32> = 1..4;
    let range: RangeInclusive<i32> = 1..=3;  // This will include the last value as well

    for num in range {
        println!({num});
    }
}
```

## Chapter - 16: Inline Modules

Inline Modules are used to organize the source code. We can group the functions, structs, enum in separate modules.
We can create a separate file for each module.

**Syntax:**
```rust
mod math {
    use std::collections::HashMap;  // If we want to use some other module
    pub fn add(a: i32, b: i32) {
        a + b
    }

    pub fn subtract(a: i32, b: i32) {
        a - b
    }
}

fn main () {
    use math::add;
    // use math::*;     // Wildcard to include all the functions of a module

    println!("{}", add(1, 5));
    println!("{}", math::subtract(1, 5));
}
```

## Chapter - 17: Testing

In Rust for testing first we need to create a test module with the test configuration `#[cfg(test)]` macro to 
compile only test module while testing and each test method should be annotated with `#[test]` macro.

Now for calling the method we need to first import the function, we can use `use crate::*` to import all the code which is in the current file.

**Syntax:**
```rust
fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

fn main() {

}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_all_caps() {
        let result = all_caps("hello");
        let expected = String::from("HELLO");
        assert_eq!(result, expected, "string should be all uppercase");
    }
}
```

## Chapter - 18: External Modules

- Modules are organized hierarchically
    - Use `super` to go up one level
    - Use `crate` to start from the top
- The `as` keyword can be used to create an alias for a module
- The `mod` keyword is used to declare a module
    - No curly braces for external modules
- Modules can be re-exported with the `use` keyword
- `pub` indicates the module may be accessed from anywhere
    - Omitting `pub` restricts access to only the containing module and sub-modules
- `pub use` in mod file is use to re-export the module to the same namespace

![external_module_example](/screenshots/external-mod.png)


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