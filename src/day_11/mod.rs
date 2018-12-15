use crate::util::*;

const GRID_SIZE: i32 = 300;

pub fn run_part_1(args: &[String]) {
    run_part_n("1", args, get_best_fuel_cell_position);
}

pub fn run_part_2(args: &[String]) {
    run_part_n("2", args, get_best_any_sized_fuel_cell_position);
}

fn get_best_fuel_cell_position(filename: &String) -> AppResult {
    let input = read_file(filename)?;
    let output = find_best_fuel_cells(input);

    println!("Best fuel cell position: {},{}", output.0, output.1);

    Ok(())
}

fn get_best_any_sized_fuel_cell_position(filename: &String) -> AppResult {
    let input = read_file(filename)?;
    let output = find_best_any_sized_fuel_cells(input);

    println!("Best fuel cell position: {},{},{}", output.0, output.1, output.2);

    Ok(())
}

fn find_best_fuel_cells(serial: i32) -> (i32, i32) {
    let grid = generate_grid(serial);
    
    let mut result = (0, 0);
    let mut result_power = std::i32::MIN;

    for y in 0..GRID_SIZE - 2 {
        for x in 0..GRID_SIZE - 2 {
            let pos_power = grid_3x3_power(x, y, &grid);
            if pos_power > result_power {
                result = (x, y);
                result_power = pos_power;
            }
        }
    }

    result
}

fn find_best_any_sized_fuel_cells(serial: i32) -> (i32, i32, i32) {
    let grid = generate_grid(serial);
    
    let mut result = (0, 0, 0);
    let mut result_power = std::i32::MIN;

    for size in 1..GRID_SIZE {
        println!("{}", size);
        for y in 0..GRID_SIZE - (size - 1) {
            for x in 0..GRID_SIZE - (size - 1) {
                let pos_power = grid_nxn_power(x, y, &grid, size);
                if pos_power > result_power {
                    result = (x, y, size);
                    result_power = pos_power;
                }
            }
        }
    }

    result
}

fn generate_grid(serial: i32) -> Vec<i32> {
    let mut result = Vec::with_capacity((GRID_SIZE * GRID_SIZE) as usize);

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            result.push(cell_power_level(x, y, serial));
        }
    }

    result
}

fn grid_3x3_power(x: i32, y: i32, grid: &Vec<i32>) -> i32 {
    grid_nxn_power(x, y, grid, 3)
}

fn grid_nxn_power(x: i32, y: i32, grid: &Vec<i32>, n: i32) -> i32 {
    GridIter::new(grid, (x, y), n).sum::<i32>()
}

fn cell_power_level(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;

    let mut power_level = rack_id * y;
    power_level += serial;
    power_level *= rack_id;
    power_level = (power_level / 100) % 10;
    power_level - 5
}

fn read_file(filename: &String) -> AppResult<i32> {
    let input = read_file_input(filename)?;
    Ok(input.parse::<i32>()?)
}

struct GridIter<'a> {
    grid: &'a Vec<i32>,
    start_pos: (i32, i32),
    current_iter: i32,
    iter_size: i32,
}

impl<'a> GridIter<'a> {
    fn new(vec: &'a Vec<i32>, start: (i32, i32), iter_size: i32) -> Self {
        GridIter {
            grid: vec,
            start_pos: start,
            current_iter: 0,
            iter_size: iter_size,
        }
    }
}

impl<'a> Iterator for GridIter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<&'a i32> {
        if self.current_iter >= self.iter_size * self.iter_size {
            return None
        }

        let mut current = self.current_iter;
        let (mut x, mut  y) = self.start_pos;

        while current >= self.iter_size {
            y += 1;
            current -= self.iter_size;
        }

        while current >= 1 {
            x += 1;
            current -= 1;
        }

        self.current_iter += 1;
        Some(&self.grid[(y * GRID_SIZE + x) as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_power_level() {
        assert_eq!(4, cell_power_level(3, 5, 8));
        assert_eq!(-5, cell_power_level(122, 79, 57));
        assert_eq!(0, cell_power_level(217, 196, 39));
        assert_eq!(4, cell_power_level(101, 153, 71));
    }

    #[test]
    fn test_find_best_fuel_cells() {
        assert_eq!((33, 45), find_best_fuel_cells(18));
        assert_eq!((21, 61), find_best_fuel_cells(42));
    }
}
