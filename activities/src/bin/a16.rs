// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

#[derive(Debug)]
struct Student {
    name: String,
    locker: Option<u8>,
}

fn main() {
    let mark = Student {
        name: "Mark".to_owned(),
        locker: Some(1),
    };

    println!("Student name: {:?}", mark.name);
    match mark.locker {
        Some(num) => println!("Locker number: {:?}", num),
        None => println!("No locker assigned"),
    }
}
