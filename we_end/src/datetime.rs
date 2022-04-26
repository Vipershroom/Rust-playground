use chrono::{Utc};

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

pub fn current_weekday() {


    let weekdays = Weekdays {
        monday: 5,
        tuesday: 4,
        wednesday: 3,
        thursday: 2,
        friday: 1,
    };


    let now = Utc::now();
    let today = (now.format("%a").to_string());
    
    let today = match today.as_str() {
        "Mon" => weekdays.monday,
        "Tue" => weekdays.tuesday,
        "Wed" => weekdays.wednesday,
        "Thu" => weekdays.thursday,
        "Fri" => weekdays.friday,
        _ => panic!("Weekday not found")
    };

    process_days(&today)
}

pub fn get_month() -> u32 {
    let now = Utc::now();
    let today = now.format("%b");
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
        _ => panic!("Couldn't get month")
    };
    
    current_month
}

pub fn get_day() -> u32 {
    let now = Utc::now();

    
    1
}

fn process_days(today: &u32) {}