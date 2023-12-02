// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

#[derive(Debug)]
struct Furniture {
    name: String,
    quantity: u8,
}

#[derive(Debug)]
struct Store {
    name: String,
    items: HashMap<u8, Furniture>,
}

impl Store {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            items: HashMap::new(),
        }
    }

    fn add_item(&mut self, id: u8, name: &str, quantity: u8) {
        self.items.insert(
            id,
            Furniture {
                name: String::from(name),
                quantity,
            },
        );
    }
}

fn main() {
    let mut store = Store::new("Furniture Store");
    store.add_item(1, "Chair", 5);
    store.add_item(2, "Bed", 3);
    store.add_item(3, "Table", 2);
    store.add_item(4, "Couch", 0);

    for (id, furniture) in store.items.iter() {
        let quantity_count = match furniture.quantity {
            0 => "out of stock".to_owned(),
            quantity => format!("{:?}", quantity),
        };

        println!(
            "Id: {id:?}, name: {:?}, quantity: {quantity_count:?}",
            furniture.name
        );
    }

    let mut total_items_in_stock = 0;
    for furniture in store.items.values() {
        if furniture.quantity > 0 {
            total_items_in_stock += 1;
        }
    }

    println!("Total items in stocks: {:?}", total_items_in_stock);
}
