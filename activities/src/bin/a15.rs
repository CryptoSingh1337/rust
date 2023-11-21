// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    BackStage(String, f64),
    Vip(String, f64),
    Standard(f64),
}

fn main() {
    let tickets = vec![
        Ticket::BackStage("Ronny".to_owned(), 120.5),
        Ticket::Vip("John".to_owned(), 225.0),
        Ticket::Standard(50.0),
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Standard(price) => println!("Standard ticket with price: {:?}", price),
            Ticket::Vip(name, price) => {
                println!("VIP Ticket with name: {:?}, price: {:?}", name, price)
            }
            Ticket::BackStage(name, price) => {
                println!("BackStage Ticket with name: {:?}, price: {:?}", name, price)
            }
        }
    }
}
