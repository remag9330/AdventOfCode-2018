use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub enum AppError {
    AppError(String),
    IOError(io::Error),
    ParseError(std::num::ParseIntError),
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::IOError(err)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(err: std::num::ParseIntError) -> Self {
        AppError::ParseError(err)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::AppError(err) => err.fmt(f),
            AppError::IOError(err) => err.fmt(f),
            AppError::ParseError(err) => err.fmt(f),
        }
    }
}

pub type AppResult<T = ()> = Result<T, AppError>;

pub fn read_file_input(filename: &String) -> Result<String, AppError> {
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn run_part_n<F>(day: &str, args: &[String], func: F)
    where F: FnOnce(&String) -> AppResult {
    match args {
        [filename] => {
            if let Err(e) = func(filename) {
                println!("Day {} Failed: {}", day, e);
            }
        },
        _ => println!("Please supply a filename as an argument to day {}", day)
    };
}
