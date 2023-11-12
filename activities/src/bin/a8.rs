// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavor {
    Sparkling,
    Sweet,
    Fruity
}

struct Drink {
    flavor: Flavor,
    fluid_ml: f32 
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavor::Sparkling => println!("The flavor is Sparkling"),
        Flavor::Sweet => println!("The flavor is Sweet"),
        Flavor::Fruity => println!("The flavor is Fruity"),
    }
    println!("The fluid in ml: {:?}", drink.fluid_ml);
}

fn main() {
    let litchi_drink = Drink {
        flavor: Flavor::Sweet,
        fluid_ml: 456.5
    };
    print_drink(litchi_drink);
    let mango_drink = Drink {
        flavor: Flavor::Fruity,
        fluid_ml: 500.0
    };
    print_drink(mango_drink);
    let sparkling_grape_drink = Drink {
        flavor: Flavor::Sparkling,
        fluid_ml: 800.0
    };
    print_drink(sparkling_grape_drink);
}
