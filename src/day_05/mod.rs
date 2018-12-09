use util;

pub fn run_part_1(args: &[String]) {
    util::run_part_n("1", args, react_polymer);
}

pub fn run_part_2(args: &[String]) {
    util::run_part_n("2", args, find_best_result);
}

fn react_polymer(filename: &String) -> util::AppResult {
    let polymer = util::read_file_input(filename)?;
    let result = perform_reaction(&polymer);

    println!("Resulting polymer length after result: {}", result.len());

    Ok(())
}

fn perform_reaction(polymer: &str) -> String {
    let mut result = String::new();

    for c in polymer.chars() {
        let prev = result.pop().unwrap_or(' ');

        if !same_char_different_case(prev, c) {
            if prev != ' ' {
                result.push(prev);
            }
            result.push(c);
        }
    }

    result
}

fn same_char_different_case(c1: char, c2: char) -> bool {
    c1 != c2 && c1.to_ascii_lowercase() == c2.to_ascii_lowercase()
}

fn find_best_result(filename: &String) -> util::AppResult {
    let polymer = util::read_file_input(filename)?;
    let result = test_all_polymers(&polymer);

    println!("Best polymer length: {}", result);

    Ok(())
}

fn test_all_polymers(polymer: &str) -> usize {
    let mut best = std::usize::MAX;

    for to_remove in "abcdefghijklmnopqrstuvwxyz".chars() {
        let test = remove_unit_from(polymer, to_remove);
        let test_result = perform_reaction(&test);

        best = std::cmp::min(test_result.len(), best);
    }

    best
}

fn remove_unit_from(polymer: &str, removing_unit: char) -> String {
    let mut result = String::new();
    let removing_unit = removing_unit.to_ascii_lowercase();

    for unit in polymer.chars() {
        if unit.to_ascii_lowercase() != removing_unit {
            result.push(unit);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_char_different_case() {
        assert_eq!(true, same_char_different_case('a', 'A'));
        assert_eq!(false, same_char_different_case('A', 'A'));
        assert_eq!(true, same_char_different_case('A', 'a'));
        assert_eq!(false, same_char_different_case('a', ' '));
    }

    #[test]
    fn test_perform_reaction() {
        assert_eq!("", perform_reaction("aA"));
        assert_eq!("", perform_reaction("abBA"));
        assert_eq!("abAB", perform_reaction("abAB"));
        assert_eq!("aabAAB", perform_reaction("aabAAB"));
        assert_eq!("dabCBAcaDA", perform_reaction("dabAcCaCBAcCcaDA"));
    }

    #[test]
    fn test_find_best_result() {
        assert_eq!(4, test_all_polymers("dabAcCaCBAcCcaDA"));
    }
}
