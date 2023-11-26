// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}

impl Adult {
    fn new(name: &str, age: u8) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                name: name.to_string(),
                age,
            })
        } else {
            Err("Person is not 21 years old")
        }
    }
}

fn print_message(person_result: Result<Adult, &str>) {
    match person_result {
        Ok(adult) => println!("{} is {} years old", adult.name, adult.age),
        Err(e) => println!("Error: {e}"),
    }
}

fn main() {
    let adult = Adult::new("Saransh", 23);
    let teen = Adult::new("Rahul", 20);

    print_message(adult);
    print_message(teen);
}
