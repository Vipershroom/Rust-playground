use chrono::{Utc};

struct Weekdays {
    monday: u32,
    tuesday: u32,
    wednesday: u32,
    thursday: u32,
    friday: u32,
}

pub fn current_day() {


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

fn process_days(today: &u32) {}