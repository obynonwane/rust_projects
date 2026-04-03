
use std::io::Stdin;
use anyhow::{Result, Error};

pub fn greet_user() -> Result<String, anyhow::Error> {
    println!("hello What is your name?");


    // declare a mutable string variable to store user input
    let mut buffer:String = String::new();

    // get a standard input handle from the standard library
    let stdin: Stdin = std::io::stdin();


    // read a line of input from the user and store it in the buffer variable
    stdin.read_line(&mut buffer)?; // means if an error occur exit the function with an error
    // check if the user's input is "ojohnson" (case-insensitive) and print a special welcome message if it is
    if buffer.trim().to_lowercase() != "ojohnson" {
        Err(Error::msg("Access denied. Only ojohnson can access this function.")) // means if the user's input is not "ojohnson" exit the function with an error containing a message with the user's input
    } else {
        Ok(buffer) // means if no error occur exit the function with the buffer variable as the result
    }

}


fn is_valid(user: &str) -> bool {
    
}