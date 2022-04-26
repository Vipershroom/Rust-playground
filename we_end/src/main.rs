use std::os::windows::process;

mod handle_json;
mod global;
mod datetime;
mod file;



fn main() {
    println!("Welcome to my H Drive maker");
    handle_json::check_prev();
    let directory = handle_json::read_json();
    let day = datetime::process_days();
    file::handle_file(directory, day);
}