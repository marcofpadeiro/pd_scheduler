mod code;
pub mod config;

use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, TimeZone, Weekday};
use chrono_tz::Europe::Lisbon;
use code::TimeRange;

use crate::config::Config;

use std::collections::HashMap;
use std::default::Default;
use std::error::Error;
use std::io::{self, Write};

use crate::code::MonthYear;

use google_calendar3::{
    api::{Event, EventDateTime},
    CalendarHub,
};
use hyper::Client;
use hyper_rustls::HttpsConnector;
use yup_oauth2::{read_application_secret, InstalledFlowAuthenticator, InstalledFlowReturnMethod};

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

    println!("\tsearch - To lookup schedule codes");
    println!("\tschedule - To create events for a month");
    println!("\thelp - Display this menu ");
}

pub async fn schedule(config: &Config) {
    let hub = auth().await.unwrap();

    let current_date = Local::now();
    let month = get_month_year(current_date.month(), MonthYear::Month);
    let year = get_month_year(u32::try_from(current_date.year()).unwrap(), MonthYear::Year);
    let weekends = get_weekends(month, year);
    let mut map: HashMap<u32, TimeRange> = HashMap::new();

    weekends.iter().for_each(|day| loop {
        print!("{day}: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().is_empty() {
            return;
        }
        if let Some(x) = TimeRange::find_code(input.trim()) {
            map.insert(*day, x);
            return;
        }
    });

    for (day, time_range) in map.iter() {
        println!("Adding {} as {}", day, time_range);
        let naive_date = NaiveDate::from_ymd_opt(year as i32, month, *day).unwrap();

        let start = NaiveDateTime::new(naive_date, time_range.start);
        let end = NaiveDateTime::new(naive_date, time_range.end);

        let start_datetime: DateTime<chrono_tz::Tz> = Lisbon.from_utc_datetime(&start);
        let end_datetime: DateTime<chrono_tz::Tz> = Lisbon.from_utc_datetime(&end);

        let start_time = start_datetime.to_rfc3339();
        let end_time = end_datetime.to_rfc3339();

        add_event(config, &hub, start_time, end_time).await.unwrap();
    }
}

async fn auth() -> Result<CalendarHub, Box<dyn Error>> {
    let secret = read_application_secret("client_secret.json").await?;

    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::Interactive)
        .persist_tokens_to_disk("tokencache.json")
        .build()
        .await?;

    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::with_native_roots());

    Ok(CalendarHub::new(client, auth))
}

async fn add_event(
    config: &Config,
    hub: &CalendarHub,
    start: String,
    end: String,
) -> Result<(), Box<dyn Error>> {
    let event = Event {
        summary: Some(config.event_name.clone().into()),
        location: Some(config.address.clone().into()),
        start: Some(EventDateTime {
            date_time: Some(start.into()),
            time_zone: Some(config.timezone.clone().into()),
            ..Default::default()
        }),
        end: Some(EventDateTime {
            date_time: Some(end.into()),
            time_zone: Some(config.timezone.clone().into()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let calendar_id = config.calendar_id.as_str();

    let result = hub.events().insert(event, calendar_id).doit().await;
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

fn get_month_year(val: u32, month_year: MonthYear) -> u32 {
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
