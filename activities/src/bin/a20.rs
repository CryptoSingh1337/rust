// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

enum PowerState {
    OFF,
    SLEEP,
    REBOOT,
    SHUTDOWN,
    HIBERNATE,
}

impl PowerState {
    fn new(state: &str) -> Option<PowerState> {
        use PowerState::*;
        let state = state.trim().to_lowercase();
        match state.as_str() {
            "off" => Some(OFF),
            "sleep" => Some(SLEEP),
            "reboot" => Some(REBOOT),
            "shutdown" => Some(SHUTDOWN),
            "hibernate" => Some(HIBERNATE),
            _ => None,
        }
    }

    fn print_power_state(state: PowerState) {
        use PowerState::*;
        match state {
            OFF => println!("Off"),
            SLEEP => println!("Sleeping"),
            SHUTDOWN => println!("Shutting down"),
            REBOOT => println!("Rebooting"),
            HIBERNATE => println!("Hibernate"),
        }
    }
}

fn get_input() -> io::Result<Option<PowerState>> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    io::Result::Ok(PowerState::new(&buffer))
}

fn main() {
    println!("Enter new power state: ");
    let input = get_input();
    match input {
        Ok(inp) => match inp {
            Some(state) => PowerState::print_power_state(state),
            None => println!("Invalid state"),
        },
        Err(e) => println!("{:?}", e),
    }
}
