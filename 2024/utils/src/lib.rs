use std::env;
use std::fs;
use std::io;

pub fn read_from_args() -> io::Result<String> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    fs::read_to_string(file_path)
}