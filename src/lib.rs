mod code;
use chrono::{Datelike, Local, NaiveDate, Weekday};

use crate::code::{MonthYear, TimeRange};
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
    let current_date = Local::now();
    let month = get_date(current_date.month(), MonthYear::Month);
    let year = get_date(u32::try_from(current_date.year()).unwrap(), MonthYear::Year);

    println!("{}/{}", month, year);
    println!("{:?}", get_weekends(month, year));
}

fn get_date(val: u32, month_year: MonthYear) -> u32 {
    loop {
        print!("Enter the {} (default: {}): ", month_year, val);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().is_empty() {
            return val;
        }

        match input.trim().parse::<u32>() {
            Ok(parsed_value) => match month_year.validate(parsed_value) {
                Ok(validated_value) => return validated_value,
                Err(_) => println!("Please enter a valid {}.", month_year),
            },
            Err(e) => println!("Invalid input: {}", e),
        }
    }
}

fn get_weekends(month: u32, year: u32) -> Vec<u32> {
    let year = i32::try_from(year).unwrap();
    let mut weekends: Vec<u32> = Vec::new();

    let first_day_of_month =
        NaiveDate::from_ymd_opt(year, month, 1).expect("Invalid first day of the month");

    let last_day_of_month = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
    }
    .expect("Invalid next month first day")
    .pred_opt();

    let mut day = first_day_of_month;
    while day <= last_day_of_month.unwrap() {
        let weekday = day.weekday();
        if weekday == Weekday::Sat || weekday == Weekday::Sun {
            weekends.push(day.day());
        }
        day = day.succ_opt().unwrap();
    }

    weekends
}

fn get_input_user_codes() -> Vec<String> {
    print!("Please enter codes divided by a space: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().split_whitespace().map(String::from).collect()
}
