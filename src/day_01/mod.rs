use std::io;

use util;

pub fn run_part_1(args: &[String]) {
    util::run_part_n("1", args, calculate_frequency);
}

pub fn run_part_2(args: &[String]) {
    util::run_part_n("1", args, calculate_repeat_frequency);
}

fn calculate_repeat_frequency(input_filename: &String) -> Result<(), io::Error> {
    let input = read_file_input(input_filename)?;
    let result = calculate_repeat(&input);

    println!("first repeat frequency: {}", result);

    Ok(())
}

fn calculate_frequency(input_filename: &String) -> Result<(), io::Error> {
    let input = read_file_input(input_filename)?;
    let result = calculate(&input);

    println!("resulting frequency: {}", result);

    Ok(())
}

fn calculate(input: &Vec<i32>) -> i32 {
    let mut acc = 0;

    for num in input.iter() {
        acc += num;
    }

    acc
}

fn calculate_repeat(input: &Vec<i32>) -> i32 {
    let mut current = 0;
    let mut cache = std::collections::HashSet::new();

    cache.insert(current);

    loop {
        for num in input.iter() {
            current += num;
            if cache.contains(&current) {
                return current;
            }

            cache.insert(current);
        }
    }
}

fn read_file_input(filename: &String) -> Result<Vec<i32>, io::Error> {
    let input = util::read_file_input(filename)?;
    let vec = parse_input(&input);

    Ok(vec)
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
