// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
    Red,
    Green,
    Blue
}

fn get_color_name(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue")
    }
}

fn main() {
    get_color_name(Color::Red);
    get_color_name(Color::Green);
    get_color_name(Color::Blue);
}
