// This code will provide a basic outline of a family counseling service

//imports 
use std::io;

// list of consts 
const MAX_COUNSELING_SESSIONS = 10;
const COUNSELING_RATE = 30;

// struct to store client information 
struct Client {
    name: String,
    address: String,
    contact: String,
    email: String
}

// empty vector to store clients
let mut clients: Vec<Client> = vec![];

// method to add clients to the list
fn add_client(name: String, address: String, contact: String, email: String) {
    let client = Client {
        name: name,
        address: address,
        contact: contact,
        email: email
    };
    clients.push(client);
}

// method to delete clients from the list
fn delete_client(index: usize) {
    clients.remove(index);
}

// method to display client list
fn display_clients() {
    for client in clients.iter() {
        println!("Name: {}, Address: {}, Contact: {}, Email: {}",
            client.name,
            client.address,
            client.contact,
            client.email
        );
    }
}

// method to start a counseling session
fn start_session() {
    println!("Welcome to the Family Counseling Service!");
    println!("How many sessions would you like to book? (Max of 10)");

    let mut sessions_booked = String::new();
    io::stdin().read_line(&mut sessions_booked)
        .expect("Failed to read line");
    let sessions_booked: u32 = sessions_booked.trim().parse()
        .expect("Please type a number!");

    if sessions_booked > MAX_COUNSELING_SESSIONS {
        println!("You have exceeded the maximum number of sessions!");
    } else {
        let total_cost = sessions_booked * COUNSELING_RATE;
        println!("Your total cost is ${}", total_cost);
        println!("Thank you for using our Family Counseling Service.");
    }
}

// main function 
fn main() {
    println!("Welcome to the Family Counseling Service!");
    println!("1. Add a Client");
    println!("2. Delete a Client");
    println!("3. Display Clients");
    println!("4. Start a Session");

    let mut selection = String::new();
    io::stdin().read_line(&mut selection)
        .expect("Failed to read line");
    let selection: u32 = selection.trim().parse()
        .expect("Please type a number!");

    match selection {
        1 => {
            println!("Enter the client's name:");
            let mut name = String::new();
            io::stdin().read_line(&mut name)
                .expect("Failed to read line");

            println!("Enter the client's address:");
            let mut address = String::new();
            io::stdin().read_line(&mut address)
                .expect("Failed to read line");

            println!("Enter the client's contact number:");
            let mut contact = String::new();
            io::stdin().read_line(&mut contact)
                .expect("Failed to read line");

            println!("Enter the client's email:");
            let mut email = String::new();
            io::stdin().read_line(&mut email)
                .expect("Failed to read line");

            add_client(name.trim().to_string(), 
                address.trim().to_string(),
                contact.trim().to_string(),
                email.trim().to_string()
            );
            println!("Client successfully added.")
        }
        2 => {
            println!("Enter the index of the client you wish to delete:");
            let mut index = String::new();
            io::stdin().read_line(&mut index)
                .expect("Failed to read line");
            let index: usize = index.trim().parse()
                .expect("Please type a number!");
            delete_client(index);
            println!("Client successfully deleted.")
        }
        3 => {
            display_clients();
        }
        4 => {
            start_session();
        }
        _ => println!("Please make a valid selection!"),
    }
}