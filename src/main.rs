use std::io;

fn main() {
    // These are the project details.
    // Bus/train ticket system (Rust)
    //
    // - Create a random Ticket ID
    // - Get the users name
    // - Get the users credit card info
    // - Ask to save information for future
    // - Provide clickable map to select destination
    // - Find the quickest route to a destination through multiple algorithms
    // - Provide selections for specific algorithm to use
    // - Save the records to an encrypted database
    // - Retrieve records from specific dates/times
    //
    // - Utilize hash table for the unique ticket information
    // - Graph algorithm for routes

    // Menu options.
    const BUS_TICKET: &str = "1";
    const TRAIN_TICKET: &str = "2";
    const RECORDS: &str = "3";
    const ADMIN: &str = "4";
    const EXIT: &str = "0";

    loop {
        // Print the Menu for the user.
        println!("Welcome to the Piccadilly bus and train ticket system.");
        println!("Main Menu\n");
        println!("Enter 1 to purchase a bus ticket.");
        println!("Enter 2 to purchase a train ticket.");
        println!("Enter 3 to retrieve ticket records.");
        println!("Enter 4 for admin access.");
        println!("Enter 0 to exit.");

        // Get the user input.
        let mut menu_selection: String = String::new();

        io::stdin()
            .read_line(&mut menu_selection)
            .expect("Failed to read line");

        // Execute the corresponding menu function.
        match menu_selection.as_str().trim() {
            BUS_TICKET => purchase_bus_ticket(),
            TRAIN_TICKET => purchase_train_ticket(),
            RECORDS => retrieve_records(),
            ADMIN => admin_access(),
            EXIT => {
                println!("Thank you for using the Piccadilly bus and train ticket system!");
                break;
            },
            _ => {
                println!("Please enter a valid selection.\n");
            },
        }
    }
}

fn purchase_bus_ticket() {
    println!("You've selected to purchase a bus ticket.");
}

fn purchase_train_ticket() {
    println!("You've selected to purchase a train ticket.");
}

fn retrieve_records() {
    println!("You've selected to retrieve records.");
}

fn admin_access() {
    println!("You've selected admin access.");
}
