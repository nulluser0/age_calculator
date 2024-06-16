use std::io::{self, stdin, Write};

use chrono::NaiveDate;

fn main() {
    // Initialize the variables
    let date_of_birth: NaiveDate; // User's date of birth
    let birthday_check_box: bool; // THe birthdayCheckBox equivalent from the thing
    println!("Age calculator"); // Title

    // ***** Request variables from user *****

    // Get user's date of birth
    loop {
        let input = &mut String::new(); // Initialize new input variable
        println!("Please enter your date of birth (DD-MM-YYYY): "); // Request user for date of birth
        io::stdout().flush().expect("Failed to flush stdout");
        stdin().read_line(input).expect("Failed to read line"); // Read line of user input

        // Parse DD-MM-YYYY to NaiveDate format
        match NaiveDate::parse_from_str(input, "%d-%m-%Y") {
            Ok(result) => {
                // Successful parse, assign parsed date to date_of_birth.
                date_of_birth = result;
                break; // Break out of loop
            }
            Err(e) => println!("Failed to parse inputted date. Please try again. {}", e), // Failed parse, loop again.
        }
    }

    // Get if user had its birthday this year.
    loop {
        let input = &mut String::new(); // Initialize new input variable
        println!("Have you had your birthday this year? (y/n): "); // Request user if user had its birthday this year.
        io::stdout().flush().expect("Failed to flush stdout");
        stdin().read_line(input).expect("Failed to read line"); // Read line

        // Match input if y, n, or neither. Not case sensitive.
        match input.trim().to_lowercase().as_str() {
            "y" => {
                // Answered "Y" or "y"
                birthday_check_box = true;
                break; // Break out of loop
            }
            "n" => {
                // Answered "N" or "n"
                birthday_check_box = false;
                break; // Break out of loop
            }
            e => println!("Unknown input: {}. Input must be y or n.", e), // Answered neither. loop again.
        }
    }

    // ***** Onto the actual logic and stuff *****
}
