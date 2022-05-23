use chrono::{Utc, Local, Datelike, Month};
use crate::global::input;

struct Weekdays {
    monday: u32,
    tuesday: u32,
    wednesday: u32,
    thursday: u32,
    friday: u32,
}

enum Months {
    January(u32),
    February(u32),
    March(u32),
    April(u32),
    May(u32),
    June(u32),
    July(u32),
    August(u32),
    September(u32),
    October(u32),
    November(u32),
    December(u32),
}

fn current_weekday() -> u32 {


    let weekdays = Weekdays {
        monday: 5,
        tuesday: 4,
        wednesday: 3,
        thursday: 2,
        friday: 1,
    };


    let now = Utc::now();
    let today = now.format("%a").to_string();
    
    let today = match today.as_str() {
        "Mon" => weekdays.monday,
        "Tue" => weekdays.tuesday,
        "Wed" => weekdays.wednesday,
        "Thu" => weekdays.thursday,
        "Fri" => weekdays.friday,
        _ => {
            println!("Program does not work on weekend\nPress enter to quit");
            input();
            panic!("Program does not work on weekend")
        }
    };
    
    today 
}

fn get_month() -> u32 {
    let now = Utc::now();

    let current_month = now.format("%b").to_string();
    let current_month = match current_month.as_str() {
        "Jan" => Months::January(1),
        "Feb" => Months::February(2),
        "Mar" => Months::March(3),
        "Apr" => Months::April(4),
        "May" => Months::May(5),
        "Jun" => Months::June(6),
        "Jul" => Months::July(7),
        "Aug" => Months::August(8),
        "Sep" => Months::September(9),
        "Oct" => Months::October(10),
        "Nov" => Months::November(11),
        "Dec" =>Months::December(12),
        _ => {
            println!("Couldn't parse month\nPress enter to quit");
            input();
            panic!("Couldn't parse month")
        }
    };
    
    match current_month {
        Months::January(value) => value,
        Months::February(value) => value,
        Months::March(value) => value,
        Months::April(value) => value,
        Months::May(value) => value,
        Months::June(value) => value,
        Months::July(value) => value,
        Months::August(value) => value,
        Months::September(value) => value,
        Months::October(value) => value,
        Months::November(value) => value,
        Months::December(value) => value,
        _ => {
            println!("Couldn't parse month\nPress enter to quit");
            input();
            panic!("Couldn't parse month")}
    }
}

fn get_day() -> u32 {
    let now = Utc::now();
    let today = now.format("%e").to_string();
    match today.trim().parse() {
        Ok(inp) => inp,
        Err(_) => {
            println!("Couldn't parse todays date\nPress enter to quit");
            input();
            panic!("Could not parse todays date number")
        }
    }
}

fn get_year() -> u32 {
    let now = Utc::now();
    let year = now.format("%Y").to_string();
    match year.parse() {
        Ok(inp) => inp,
        Err(_) => {
            println!("Could not parse year\nPress enter to quit");
            input();
            panic!("Could not parse year")
        }
    }
}

pub fn process_days() -> String {
    let weekday = current_weekday();
    let month = get_month();
    let year = get_year();
    let mut day = get_day();

    match weekday {
        5 => {day += 4},
        4 => {day += 3},
        3 => {day += 2},
        2 => {day += 1},
        1 => {todo!()},
        _ => {
            println!("Error processing days\nPress enter to quit");
            input();
            panic!("Error processing days")
        }
    }
    let folder_date = format!("WE_{}_{}_{}",month, day, year );

    folder_date
}