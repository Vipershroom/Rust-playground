use clap::Parser;
use std::path::{Path, PathBuf};
use chrono::{NaiveDate, Weekday, Datelike};

fn week_bounds(week: u32) -> (NaiveDate, NaiveDate) {
    let current_year = chrono::offset::Local::now().year();
    let mon = NaiveDate::from_isoywd(current_year, week, Weekday::Mon);
    let sun = NaiveDate::from_isoywd(current_year, week, Weekday::Sun);
    (mon, sun)
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value_t= String::from("WHAT"))]
    name: String,

    #[clap(short, long, action)]
    verbose: bool,

    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,
}


fn main() {
    let args = Args::parse();
    let d =NaiveDate::from_isoywd(2015, 15, Weekday::Mon);
    let (mon,sun) = week_bounds(d.iso_week().week());
    println!("{:?}   {:?}", mon, sun);
}
