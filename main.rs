/*
Name: Jon Bennett
Date: 03/03/24
Class: 301

Discription:
In this progam you will have the option from selecting between address, log events, and phone number. This will then based
on your selection search through the text file to location the following information and print out to the user utilizing 
the regex crate and format to filter. 
*/

use regex::Regex;
use std::fs;
use std::io;

fn search_addresses() {
    // Define the regular expression pattern for an address
    let address_regex = Regex::new(r"\d+\s+[\w\s\d.,'()-]+\s*,\s*[\w\s\d.,'()-]+\s*,\s*[\w\s\d.,'()-]+\s*,?\s*[\w\s\d.,'()-]*").unwrap();

    // Read the text file
    let file_content = fs::read_to_string("Phone_and_address.txt").expect("Failed to read file");

    // search through each line for addresses
    let mut found_addresses = Vec::new();
    for line in file_content.lines() {
        if address_regex.is_match(line) {
            found_addresses.push(line.to_owned());
        }
    }

    // Print out found addresses
    if found_addresses.is_empty() {
        println!("No addresses found in the text file.");
    } else {
        println!("Found addresses:");
        for address in found_addresses {
            println!("{}", address);
        }
    }
}

fn search_log_events() {
    // Read the log file
    let file_content = fs::read_to_string("log.txt").expect("Failed to read file");

    // Define regular expressions for each event
    let failed_login_regex = Regex::new(r"(\w{3}\s+\d{1,2} \d{2}:\d{2}:\d{2}).*?Failed password for (\w+) from (\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})").unwrap();
    let successful_login_regex = Regex::new(r"(\w{3}\s+\d{1,2} \d{2}:\d{2}:\d{2}).*?Accepted password for (\w+) from (\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})").unwrap();
    let sudo_command_regex = Regex::new(r"(\w{3}\s+\d{1,2} \d{2}:\d{2}:\d{2}).*?COMMAND=(.+)$").unwrap();

    // search through each line for events 
    let mut found_events = Vec::new();
    for line in file_content.lines() {
        if let Some(captures) = failed_login_regex.captures(line) {
            let timestamp = captures.get(1).map_or("", |m| m.as_str());
            let username = captures.get(2).map_or("", |m| m.as_str());
            let ip_address = captures.get(3).map_or("", |m| m.as_str());
            found_events.push(format!("Failed login attempt: Timestamp={}, Username={}, IP={}", timestamp, username, ip_address));
        } else if let Some(captures) = successful_login_regex.captures(line) {
            let timestamp = captures.get(1).map_or("", |m| m.as_str());
            let username = captures.get(2).map_or("", |m| m.as_str());
            let ip_address = captures.get(3).map_or("", |m| m.as_str());
            found_events.push(format!("Successful login: Timestamp={}, Username={}, IP={}", timestamp, username, ip_address));
        } else if let Some(captures) = sudo_command_regex.captures(line) {
            let timestamp = captures.get(1).map_or("", |m| m.as_str());
            let command = captures.get(2).map_or("", |m| m.as_str());
            found_events.push(format!("Sudo command executed: Timestamp={}, Command={}", timestamp, command));
        }
    }

    // Print out found events
    if found_events.is_empty() {
        println!("No events found in the log file.");
    } else {
        println!("Found events:");
        for event in found_events {
            println!("{}", event);
        }
    }
}

fn search_phone_numbers() {
    // Read the text file
    let file_content = fs::read_to_string("Phone_and_address.txt").expect("Failed to read file");

    // Define regular expression for phone numbers
    let phone_regex = Regex::new(r"(?:\+?\d{1,3}\s*)?(?:[\(\s-]?\d{2,3}[\)\s-]?\s*)?\d{3,4}(?:[\s-]?\d{2,4}){1,2}").unwrap();

    // Iterate through each line and search for phone numbers
    let mut found_numbers = Vec::new();
    for line in file_content.lines() {
        for captures in phone_regex.captures_iter(line) {
            let number = captures[0].to_string();
            found_numbers.push(number);
        }
    }

    // Print out found phone numbers
    if found_numbers.is_empty() {
        println!("No phone numbers found in the text file.");
    } else {
        println!("Found phone numbers:");
        for number in found_numbers {
            println!("{}", number);
        }
    }
}

fn main() {
    loop {
        // Ask the user what they want to search for
        println!("What do you want to search for? Enter 'address', 'event', or 'phone#':");

        let mut search_option = String::new();
        io::stdin().read_line(&mut search_option).expect("Failed to read line");
        let search_option = search_option.trim().to_lowercase();

        // Perform search based on user's choice
        match search_option.as_str() {
            "address" => search_addresses(),
            "event" => search_log_events(),
            "phone#" => search_phone_numbers(),
            _ => println!("Invalid option. Please enter 'address', 'event', or 'phone#'."),
        }

        // Ask if the user wants to rerun the program
        println!("Do you want to perform another search? yes/no. Anything thing else will exit assuming no.");

        let mut rerun_option = String::new();
        io::stdin().read_line(&mut rerun_option).expect("Failed to read line");
        let rerun_option = rerun_option.trim().to_lowercase();

        if rerun_option != "yes" {
            break;
        }
    }
}


