mod helper;

use helper::*;
use std::collections::HashMap;

fn main_options() {
    println!("1. Add bill");
    println!("2. View bill");
    println!("3. Remove bill");
    println!("4. Update bill");
    println!("5. Bill total");
    println!("6. Print options");
}

fn main() {
    let mut bills: HashMap<String, Bill> = HashMap::new();
    println!("== Manage Bills ==");
    main_options();
    loop {
        println!("Enter selection: ");
        let input: u8 = get_input().unwrap().parse().unwrap();
        match input {
            1 => add(&mut bills),
            2 => view(&bills),
            3 => remove(&mut bills),
            4 => update(&mut bills),
            5 => total_bill(&bills),
            6 => main_options(),
            _ => break,
        }
    }
}
