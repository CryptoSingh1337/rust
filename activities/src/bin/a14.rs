// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    age: u8,
    favourite_color: String,
}

impl Person {
    fn new(name: &str, age: u8, color: &str) -> Self {
        Self {
            name: String::from(name),
            age,
            favourite_color: String::from(color),
        }
    }

    fn display(&self) {
        println!("Name: {:?}", self.name);
        println!("Age: {:?}", self.age);
        println!("Color: {:?}", self.favourite_color);
    }
}

fn main() {
    let persons = vec![
        Person::new("Chris", 8, "Purple"),
        Person::new("William", 9, "Red"),
        Person::new("Jacky", 20, "Blue"),
    ];

    for person in persons {
        if person.age <= 10 {
            person.display();
        }
    }
}
