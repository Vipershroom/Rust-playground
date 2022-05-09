use chrono::{Utc, Local, Datelike};
use crate::global::input;

struct Weekdays {
    monday: u32,
    tuesday: u32,
    wednesday: u32,
    thursday: u32,
    friday: u32,
}

struct Months {
    january: u32,
    february: u32,
    march: u32,
    april: u32,
    may: u32,
    june: u32,
    july: u32,
    august: u32,
    september: u32,
    october: u32,
    november: u32,
    december: u32,
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
    let months = Months {
        january: 1,
        february: 2,
        march: 3,
        april: 4,
        may: 5,
        june: 6,
        july: 7,
        august: 8,
        september: 9,
        october: 10,
        november: 11,
        december: 12,
    };

    let current_month = now.format("%b").to_string();
    let current_month = match current_month.as_str() {
        "Jan" => months.january,
        "Feb" => months.february,
        "Mar" => months.march,
        "Apr" => months.april,
        "May" => months.may,
        "Jun" => months.june,
        "Jul" => months.july,
        "Aug" => months.august,
        "Sep" => months.september,
        "Oct" => months.october,
        "Nov" => months.november,
        "Dec" => months.december,
        _ => {
            println!("Couldn't parse month\nPress enter to quit");
            input();
            panic!("Couldn't parse month")
        }
    };
    
    current_month
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