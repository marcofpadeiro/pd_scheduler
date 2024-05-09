mod code;
use crate::code::TimeRange;
use std::collections::HashMap;
use std::io::{self, Write};

pub fn search(mut codes: Option<Vec<String>>) {
    let mut valid_input = false;
    let mut invalid_codes: Vec<String> = Vec::new();
    let mut map: HashMap<String, TimeRange> = HashMap::new();

    while !valid_input {
        let current_codes = match codes {
            Some(ref c) => c,
            None => {
                let user_codes = get_input_user_codes();
                codes = Some(user_codes);
                codes.as_ref().unwrap()
            }
        };

        map.clear();

        current_codes
            .iter()
            .for_each(|code| match TimeRange::find_code(code) {
                Some(timerange) => {
                    map.insert(code.clone(), timerange);
                }
                None => {
                    invalid_codes.push(code.clone());
                    return;
                }
            });

        if map.len() == current_codes.len() {
            valid_input = true;
        } else if codes.is_some() {
            println!("Invalid codes: {:?}", invalid_codes);
            return;
        } else {
            println!("Invalid codes: {:?}", invalid_codes);
        }
    }

    map.iter().for_each(|(key, val)| {
        println!("{} => {}", key, val);
    });
}

pub fn help() {
    println!("Usage: ./pd_scheduler <action> [args]");
    println!("Possible Actions:");

    println!("\tsetup - To setup your Google Calendar account");
    println!("\tsearch - To lookup schedule codes");
    println!("\tschedule - To create events for a month");
    println!("\thelp - Display this menu ");
}

pub fn setup() {
    todo!()
}

pub fn schedule() {
    todo!()
}

fn get_input_user_codes() -> Vec<String> {
    print!("Please enter codes divided by a space: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().split_whitespace().map(String::from).collect()
}
