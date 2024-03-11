use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
pub struct Bill {
    pub title: String,
    pub amount: f64,
}

impl Bill {
    pub fn new(title: &str, amount: f64) -> Self {
        Self {
            title: title.to_owned(),
            amount,
        }
    }
}

pub fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

pub fn add(bills: &mut HashMap<String, Bill>) {
    println!("Enter the title: ");
    let title = get_input().unwrap();
    println!("Enter the amount: ");
    let amount = get_input().unwrap().parse::<f64>().unwrap();
    let bill = Bill::new(&title, amount);
    bills.insert(bill.title.clone(), bill);
}

pub fn view(bills: &HashMap<String, Bill>) {
    println!("Bills: ");
    for bill in bills {
        println!("{:?}", bill.1);
    }
}

pub fn remove(bills: &mut HashMap<String, Bill>) {
    println!("Enter the title to remove: ");
    let title = get_input().unwrap();
    bills.remove(&title);
}

pub fn update(bills: &mut HashMap<String, Bill>) {
    println!("Enter the title to update: ");
    let title = get_input().unwrap();
    println!("Enter the new amount: ");
    let amount = get_input().unwrap().parse::<f64>().unwrap();
    match bills.entry(title.to_owned()) {
        Entry::Occupied(mut entry) => {
            entry.get_mut().amount = amount;
            println!("Updated the bill");
        }
        Entry::Vacant(_) => println!("Bill: {title} not found"),
    };
}

pub fn total_bill(bills: &HashMap<String, Bill>) {
    let mut total: f64 = 0.0;
    for bill in bills {
        total += bill.1.amount
    }
    println!("Total bill: {total}");
}
