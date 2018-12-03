use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn read_file_input(filename: &String) -> Result<String, io::Error> {
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn run_part_n<F>(day: &str, args: &[String], func: F)
    where F: FnOnce(&String) -> Result<(), io::Error> {
    match args {
        [filename] => {
            if let Err(e) = func(filename) {
                println!("Day {} Failed: {}", day, e);
            }
        },
        _ => println!("Please supply a filename as an argument to day {}", day)
    };
}
