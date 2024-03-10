// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::{DateTime, Local};

fn main() {
    let current_time: DateTime<Local> = Local::now();
    println!(
        "Current date time: {:?}",
        current_time.format("%d-%m-%Y %H:%M:%S").to_string()
    );
}
