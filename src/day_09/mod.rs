use std::num::{NonZeroU32};

use crate::util::*;

use linked_list::{LinkedList, Cursor};

pub fn run_part_1(args: &[String]) {
    run_part_n("1", args, calculate_winner);
}

pub fn run_part_2(args: &[String]) {
    run_part_n("2", args, calculate_bigger_winner);
}

fn calculate_winner(filename: &String) -> AppResult {
    let (player_count, max_marble_score) = read_input(filename)?;
    let results = simulate_game(player_count, max_marble_score);
    
    println!("Highest score: {}", results.iter().max().unwrap_or(&0));

    Ok(())
}

fn calculate_bigger_winner(filename: &String) -> AppResult {
    let (player_count, max_marble_score) = read_input(filename)?;
    let results = simulate_game(player_count, max_marble_score * 100);
    
    println!("Highest score: {}", results.iter().max().unwrap_or(&0));

    Ok(())
}

fn simulate_game(player_count: usize, max_marble_score: usize) -> Vec<usize> {
    let mut player_score = vec!(0; player_count);
    let mut marbles = LinkedList::new();
    marbles.push_back(0);
    let mut cursor = marbles.cursor();

    let mut current_player = 0;
 
    for marble_score in 1..=max_marble_score {
        if marble_score % 23 == 0 {
            let remove_value = counter_clockwise_step_n(&mut cursor, NonZeroU32::new(7).unwrap());
            *player_score.get_mut(current_player).unwrap() += *remove_value;
            *player_score.get_mut(current_player).unwrap() += marble_score;
            cursor.remove();
        } else {
            clockwise_step_n(&mut cursor, NonZeroU32::new(2).unwrap());
            cursor.insert(marble_score);

        }

        current_player = (current_player + 1) % player_count;
    }

    player_score
}

fn clockwise_step_n<'a, T: 'a>(c: &'a mut Cursor<T>, num: NonZeroU32) -> &'a mut T {
    for _ in 0..num.get() - 1 {
        clockwise_step(c);
    }

    clockwise_step(c)
}

fn clockwise_step<'a, T: 'a>(c: &'a mut Cursor<T>) -> &'a mut T {
    c.next();
    if c.peek_next().is_none() {
        c.next();
    }

    c.peek_next().unwrap()
}

fn counter_clockwise_step_n<'a, T: 'a>(c: &'a mut Cursor<T>, num: NonZeroU32) -> &'a mut T {
    for _ in 0..num.get() - 1 {
        counter_clockwise_step(c);
    }

    counter_clockwise_step(c)
}

fn counter_clockwise_step<'a, T: 'a>(c: &'a mut Cursor<T>) -> &'a mut T {
    if c.prev().is_none() {
        c.prev();
    }

    c.peek_next().unwrap()
}

fn read_input(filename: &String) -> AppResult<(usize, usize)> {
    let input = read_file_input(filename)?;
    let split = input.split(" ").collect::<Vec<&str>>();

    match split.as_slice() {
        [players, "players;", "last", "marble", "is", "worth", max_points, "points"] => {
            let players = players.parse::<usize>()?;
            let max_points = max_points.parse::<usize>()?;
            Ok((players, max_points))
        },
        _ => Err(AppError::AppError(String::from("Could not parse file input")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_game() {
        assert_eq!(8317, *simulate_game(10, 1618).iter().max().unwrap());
        assert_eq!(146373, *simulate_game(13, 7999).iter().max().unwrap());
        assert_eq!(2764, *simulate_game(17, 1104).iter().max().unwrap());
        assert_eq!(54718, *simulate_game(21, 6111).iter().max().unwrap());
        assert_eq!(37305, *simulate_game(30, 5807).iter().max().unwrap());
    }
}
