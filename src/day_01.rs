use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn run_part_1(args: &[String]) {
    match args {
        [filename] => {
            if let Err(e) = calculate_frequency(filename) {
                println!("Day 1 Failed: {}", e);
            }
        },
        _ => println!("Please supply a filename as an argument to day 1")
    };
}

fn calculate_frequency(input_filename: &String) -> Result<(), io::Error> {
    let input = read_file_input(input_filename)?;
    let result = calculate(input);

    println!("resulting frequency: {}", result);

    Ok(())
}

fn calculate(input: Vec<i32>) -> i32 {
    let mut acc = 0;

    for num in input.iter() {
        acc += num;
    }

    acc
}

fn read_file_input(filename: &String) -> Result<Vec<i32>, io::Error> {
    let input = read_file(filename)?;
    let vec = parse_input(&input);

    Ok(vec)
}

fn read_file(filename: &String) -> Result<String, io::Error> {
    let mut f = File::open(filename)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;

    Ok(input)
}

fn parse_input(input: &String) -> Vec<i32> {
    let mut result = Vec::new();

    for line in input.lines() {
        match line.parse::<i32>() {
            Ok(num) => result.push(num),
            Err(e) => println!("Invalid value in input: {}", e)
        };
    }

    result
}
