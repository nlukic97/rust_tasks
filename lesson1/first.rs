use std::io;

fn main() {

    // Print a message to ask for the user's name
    
    println!("Enter your name:");
    
    let mut name = String::new();
    
    io::stdin().read_line(&mut name).expect("Failed to read line.");
    
    let name =name.trim();
    
    println!("Hello {}!",name);
    println!("Hello {name}!"); // strings are utf-8 by default.
}