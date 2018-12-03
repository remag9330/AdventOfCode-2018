use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn run_part_1(args: &[String]) {
    match args {
        [filename] => {
            if let Err(e) = calculate_checksum(filename) {
                println!("Day 2 Failed: {}", e);
            }
        },
        _ => println!("Please supply a filename as an argument to day 2")
    };
}

pub fn run_part_2(args: &[String]) {
    match args {
        [filename] => {
            if let Err(e) = find_similar_characters(filename) {
                println!("Day 2 Failed: {}", e);
            }
        },
        _ => println!("Please supply a filename as an argument to day 2")
    };
}

fn calculate_checksum(filename: &String) -> Result<(), io::Error> {
    let input = read_file_input(filename)?;
    let result = calculate_checksums(&input);

    println!("The checksum is {}", result);

    Ok(())
}

fn find_similar_characters(filename: &String) -> Result<(), io::Error> {
    let input = read_file_input(filename)?;
    let result = calculate_similar(&input);

    match result {
        Some(result) => println!("The common characters are {}", result),
        None => println!("No matching IDs.")
    };

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

fn calculate_similar(input: &String) -> Option<String> {
    for (line1, line2) in iterate_all_lines_against_each_other(input) {
        if let Some(matching) = compare_ids(&line1, &line2) {
            return Some(matching)
        }
    }

    None
}

fn compare_ids(line1: &String, line2: &String) -> Option<String> {
    if line1.len() != line2.len() {
        return None;
    }

    let mut mismatches = 0;

    for (c1, c2) in line1.chars().zip(line2.chars()) {
        if c1 != c2 {
            mismatches += 1;
        }
    }

    if mismatches == 1 {
        Some(find_common_substring(line1, line2))
    } else {
        None
    }
}

fn find_common_substring(line1: &String, line2: &String) -> String {
    let mut result = String::new();

    for (c1, c2) in line1.chars().zip(line2.chars()) {
        if c1 == c2 {
            result.push(c1);
        }
    }

    result
}

fn iterate_all_lines_against_each_other(input: &String) -> LineComparerIterator {
    LineComparerIterator::new(input)
}

struct LineComparerIterator {
    lines: Vec<String>,
    current_index: usize,
    compare_index: usize,
}

impl LineComparerIterator {
    fn new(string: &String) -> LineComparerIterator {
        LineComparerIterator {
            lines: string.clone().lines().map(|x| String::from(x)).collect(),
            current_index: 0,
            compare_index: 0,
        }
    }
}

impl Iterator for LineComparerIterator {
    type Item = (String, String);

    fn next(&mut self) -> Option<(String, String)> {
        self.compare_index += 1;
        if self.compare_index >= self.lines.len() {
            self.current_index += 1;
            self.compare_index = self.current_index + 1;
        }

        if self.current_index >= self.lines.len() - 1 {
            None
        } else {
            Some((self.lines[self.current_index].clone(), self.lines[self.compare_index].clone()))
        }
    }
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

    #[test]
    fn test_calculate_similar() {
        let s = String::from("abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz");
        let result = calculate_similar(&s);
        assert_eq!(result.unwrap(), String::from("fgij"));
    }
}
