use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;
use std::fs;
use std::panic;

use crate::global::input;

pub fn handle_file(directory: String, day: String) {
    let options = CopyOptions::new();

    match copy("WE_Template",&directory, &options) {
        Ok(cop) => cop,
        Err(_) => {
            println!("Failed to copy to directory\nPress enter to continue");
            input();
            panic!("Failed to copy to directory")
        }
    };

    let format_dir = format!("{}\\WE_Template", &directory);
    let format_dir2 = format!("{}\\{}", &directory, day);

    match fs::rename(format_dir, format_dir2) {
        Ok(ren) => ren,
        Err(_) => println!("WARNING, Couldn't rename file")
    };
}