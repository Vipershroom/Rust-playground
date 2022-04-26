use fs_extra::dir::copy;
use fs_extra::dir::CopyOptions;
use std::fs;

pub fn handle_file(directory: String, day: String) {
    let options = CopyOptions::new();

    copy("WE_Template",&directory, &options).expect("Couldn't copy directory");

    let format_dir = format!("{}\\WE_Template", &directory);
    let format_dir2 = format!("{}\\{}", &directory, day);

    fs::rename(format_dir, format_dir2).expect("Couldn't rename file");
}