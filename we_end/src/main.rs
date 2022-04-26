mod handle_json;
mod global;
mod datetime;



fn main() {
    println!("Welcome to my H Drive maker");
    handle_json::check_prev();
    let directory = handle_json::read_json();
    datetime::get_month();
}