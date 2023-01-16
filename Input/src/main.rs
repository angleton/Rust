use std::io;
use std::thread;

fn main() {
    // Ask the user for their name
    let mut name = String::new();
    println!("What is your name?");
    io::stdin().read_line(&mut name).unwrap();
    // Trim the newline character from the end of the input
    name = name.trim().to_string();
    // Create a new thread to reverse the name
    let handle = thread::spawn(move || {
        let reversed_name = name.chars().rev().collect::<String>();
        reversed_name
    });
    // Wait for the thread to finish and get the result
    let reversed_name = handle.join().unwrap();
    // Output the result
    println!("Your name reversed is: {}", reversed_name);
}