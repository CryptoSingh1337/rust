// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    RED,
    GREEN,
    BLUE,
}

impl Color {
    fn print(&self) {
        println!("Color:");
        match self {
            Color::RED => println!("\tRED"),
            Color::GREEN => println!("\tGREEN"),
            Color::BLUE => println!("\tBLUE"),
        }
    }
}

struct Dimensions {
    length: f32,
    width: f32,
    height: f32,
}

impl Dimensions {
    fn print(&self) {
        println!("Dimensions:");
        println!("\tLength: {:?} cm", self.length);
        println!("\tWidth: {:?} cm", self.width);
        println!("\tHeight: {:?} cm", self.height);
    }
}

struct ShippingBox {
    dimensions: Dimensions,
    weight: f32,
    color: Color,
}

impl ShippingBox {
    fn new(dimensions: Dimensions, weight: f32, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print_characterstics(&self) {
        self.dimensions.print();
        println!("Weight:\n\t{:?} kg", self.weight);
        self.color.print();
    }
}

fn main() {
    println!("############### BOX - 1 ###############");
    let shipping_box = ShippingBox::new(
        Dimensions {
            length: 15.0,
            width: 30.0,
            height: 10.0,
        },
        15.0,
        Color::RED,
    );
    shipping_box.print_characterstics();

    println!("############### BOX - 2 ###############");
    let shipping_box = ShippingBox::new(
        Dimensions {
            length: 15.5,
            width: 30.5,
            height: 10.5,
        },
        15.5,
        Color::GREEN,
    );
    shipping_box.print_characterstics();

    println!("############### BOX - 3 ###############");
    let shipping_box = ShippingBox::new(
        Dimensions {
            length: 16.0,
            width: 31.0,
            height: 11.0,
        },
        16.0,
        Color::BLUE,
    );
    shipping_box.print_characterstics();
}
