use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn run_part_1(args: &[String]) {
    match args {
        [filename] => {
            if let Err(e) = calculate_checksum(filename) {
                println!("Day 1 Failed: {}", e);
            }
        },
        _ => println!("Please supply a filename as an argument to day 1")
    };
}

fn calculate_checksum(filename: &String) -> Result<(), io::Error> {
    let input = read_file_input(filename)?;
    let result = calculate_checksums(&input);

    println!("The checksum is {}", result);

    Ok(())
}

fn calculate_checksums(input: &String) -> i32 {
    let mut twos = 0;
    let mut threes = 0;

    for word in input.lines() {
        let counter = count_letters(word);

        if counter.values().any(|&x| x == 2) {
            twos += 1;
        }

        if counter.values().any(|&x| x == 3) {
            threes += 1;
        }
    }

    println!("{}, {}", twos, threes);

    twos * threes
}

fn count_letters(word: &str) -> HashMap<char, i32> {
    let mut result = HashMap::new();

    for letter in word.chars() {
        *result.entry(letter).or_insert(0) += 1;
    }

    result
}

fn read_file_input(filename: &String) -> Result<String, io::Error> {
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_checksums() {
        let result = calculate_checksums(&String::from("abc\naabccc\naaabbc"));
        assert_eq!(result, 4);
    }

    #[test]
    fn test_count_letters() {
        let r1 = count_letters("aaabcc");
        assert_eq!(r1.get(&"a".chars().next().unwrap()).unwrap(), &3);
        assert_eq!(r1.get(&"b".chars().next().unwrap()).unwrap(), &1);
        assert_eq!(r1.get(&"c".chars().next().unwrap()).unwrap(), &2);
    }

    #[test]
    fn test_count_letters_check() {
        let r1 = count_letters("aaabcc");

        assert!(r1.values().any(|&x| x == 2));
        assert!(r1.values().any(|&x| x == 3));
    }
}
