use std::io; // Import the io module from the standard library

fn main() {
    // Print a message to prompt the user for the first number
    println!("Enter the first number:");

    // Declare a mutable variable to store the first number
    let mut input1 = String::new(); // Create a new empty String to store user input
    io::stdin().read_line(&mut input1).expect("Failed to read line"); // Read user input

    // Convert the input to a number (integer)
    let num1: i32 = input1.trim().parse().expect("Please enter a valid number");

    // Repeat the process for the second number
    println!("Enter the second number:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let num2: i32 = input2.trim().parse().expect("Please enter a valid number");

    // Calculate the sum
    let sum = num1 + num2;

    // Print the result
    println!("The sum of {} and {} is: {}", num1, num2, sum);
}
