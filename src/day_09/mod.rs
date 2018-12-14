use util::*;

pub fn run_part_1(args: &[String]) {
    run_part_n("1", args, calculate_winner);
}

pub fn run_part_2(args: &[String]) {
    run_part_n("2", args, |_| Ok(()));
}

fn calculate_winner(filename: &String) -> AppResult {
    let (player_count, max_marble_score) = read_input(filename)?;
    let results = simulate_game(player_count, max_marble_score);
    
    println!("Highest score: {}", results.iter().max().unwrap_or(&0));

    Ok(())
}

fn simulate_game(player_count: usize, max_marble_score: usize) -> Vec<usize> {
    let mut player_score = vec!(0; player_count);
    let mut marbles = vec!(0);

    let mut current_marble = 0;
    let mut current_player = 0;
 
    for marble_score in 1..=max_marble_score {
        if marble_score % 23 == 0 {
            let remove_index = counter_clockwise_index(&marbles, current_marble, 7);
            *player_score.get_mut(current_player).unwrap() += marbles.remove(remove_index);
            *player_score.get_mut(current_player).unwrap() += marble_score;
            current_marble = remove_index;
        } else {
            let left_insert_index = clockwise_index(&marbles, current_marble, 1);
            let right_insert_index = clockwise_index(&marbles, current_marble, 2);

            if left_insert_index == marbles.len() - 1 && right_insert_index == 0 {
                marbles.push(marble_score);
                current_marble = marbles.len() - 1;
            } else {
                marbles.insert(right_insert_index, marble_score);
                current_marble = right_insert_index;
            }
        }

        current_player = (current_player + 1) % player_count;
    }

    player_score
}

fn clockwise_index(marbles: &Vec<usize>, current: usize, count: usize) -> usize {
    (current + count) % marbles.len()
}

fn counter_clockwise_index(marbles: &Vec<usize>, current: usize, count: usize) -> usize {
    (current + marbles.len() - count) % marbles.len()
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
