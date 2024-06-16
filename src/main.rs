use std::io::{self, stdin, Write};

use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

fn main() {
    println!("To quit, press CTRL + C (^C).");
    println!("Age calculator"); // Title
    loop {
        // Initialize the variables
        let date_of_birth: NaiveDateTime; // User's date of birth as NaiveDateTime
        let birthday_check_box: bool; // THe birthdayCheckBox equivalent from the thing (wtf is the use of this)

        // ***** Request variables from user *****

        // Get user's date of birth
        loop {
            let input = &mut String::new(); // Initialize new input variable
            print!("Please enter your date of birth (DD-MM-YYYY): "); // Request user for date of birth
            io::stdout().flush().expect("Failed to flush stdout");
            stdin().read_line(input).expect("Failed to read line"); // Read line of user input

            // Parse DD-MM-YYYY to NaiveDate format
            match NaiveDate::parse_from_str(input.trim(), "%d-%m-%Y") {
                Ok(result) => {
                    // Successful parse, assign parsed date to date_of_birth.
                    date_of_birth = result
                        .and_time(NaiveTime::from_num_seconds_from_midnight_opt(0, 0).unwrap()); // Add midnight to make it NaiveDateTime instead of NaiveDate
                    break; // Break out of loop
                }
                Err(e) => println!("Failed to parse inputted date. Please try again. {}", e), // Failed parse, loop again.
            }
        }

        // Get if user had its birthday this year.
        loop {
            let input = &mut String::new(); // Initialize new input variable
            print!("Have you had your birthday this year? (y/n): "); // Request user if user had its birthday this year.
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
        let current_date: NaiveDateTime = chrono::offset::Local::now().naive_local(); // Get current time, and convert to NaiveDateTime

        // Get difference in years, months, days
        let mut years: i32 = current_date.year() - date_of_birth.year();
        let mut months: i32 = current_date.month() as i32 - date_of_birth.month() as i32;
        let mut days: i32 = current_date.day() as i32 - date_of_birth.day() as i32;
        let mut hours: i32 = current_date.hour() as i32 - date_of_birth.hour() as i32;
        let mut minutes: i32 = current_date.minute() as i32 - date_of_birth.minute() as i32;
        let mut seconds: i32 = current_date.second() as i32 - date_of_birth.second() as i32;

        // Handle cases where these dates are negative (I have made helper functions to make it cleaner)
        adjust_negative_values(&mut seconds, &mut minutes, 60);
        adjust_negative_values(&mut minutes, &mut hours, 60);
        adjust_negative_values(&mut hours, &mut days, 24);
        if days < 0 {
            months -= 1;
            days += days_in_previous_month(current_date);
        }
        if months < 0 {
            years -= 1;
            months += 12;
        }
        adjust_negative_values(&mut months, &mut years, 12);

        // Handle the birthday_check_box
        // Personally, this is plain stupid for people to include, since the DOB logic is already implemented.
        // Regardless, I'll leave the code here, it will just be commented out for now since it's dumb.
        // match birthday_check_box {
        //     true => {}
        //     false => years -= 1,
        // }

        // Print results
        println!(
            "The current time is: {}",
            current_date.format("%H:%M:%S %d-%m-%Y")
        );
        println!(
            "Your age is: {} years, {} months, {} days, {} hours, {} minutes, {} seconds.",
            years, months, days, hours, minutes, seconds
        );
        if current_date.month() == date_of_birth.month()
            && current_date.day() == date_of_birth.day()
        {
            // It is user's birthday.
            println!("Happy Birthday!")
        }
    }
}

fn adjust_negative_values(current: &mut i32, higher: &mut i32, modulus: i32) {
    if *current < 0 {
        *higher -= 1;
        *current += modulus;
    }
}

fn days_in_previous_month(date: NaiveDateTime) -> i32 {
    // bad code
    if date.month() == 1 {
        // January - so the previous month is December of the previous year
        NaiveDate::from_ymd_opt(date.year() - 1, 12, 1)
            .expect("Failed to create new date")
            .and_hms_opt(0, 0, 0)
            .expect("Failed to add hms to date")
            .with_month0(12)
            .unwrap()
            .signed_duration_since(
                NaiveDate::from_ymd_opt(date.year() - 1, 12, 1)
                    .expect("Failed to get signed duration")
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            )
            .num_days() as i32
    } else {
        // Any other month
        NaiveDate::from_ymd_opt(date.year(), date.month() - 1, 1)
            .expect("Failed to create new date")
            .and_hms_opt(0, 0, 0)
            .expect("Failed to add hms to date")
            .with_month0(
                (date.month() as i32 - 1)
                    .try_into()
                    .expect("Failed to convert i32 to u32"),
            )
            .unwrap()
            .signed_duration_since(
                NaiveDate::from_ymd_opt(date.year(), date.month() - 1, 1)
                    .expect("Failed to get signed duration")
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            )
            .num_days() as i32
    }
}
