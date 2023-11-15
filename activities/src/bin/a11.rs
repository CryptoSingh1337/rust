// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    id: u32,
    quantity: u32,
}

fn display_id(grocery: &Grocery) {
    println!("Id: {:?}", grocery.id);
}

fn display_quantity(grocery: &Grocery) {
    println!("Quantity: {:?}", grocery.quantity);
}

fn main() {
    let potato = Grocery {
        id: 1,
        quantity: 10,
    };
    display_id(&potato);
    display_quantity(&potato);
}
