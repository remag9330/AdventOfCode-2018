use crate::util::*;

pub fn run_part_1(args: &[String]) {
    run_part_n("1", args, calculate_word);
}

pub fn run_part_2(args: &[String]) {
    run_part_n("2", args, |_| Ok(()));
}

fn calculate_word(filename: &String) -> AppResult {
    let mut input = read_input(filename)?;
    let output = find_most_likely_output(&mut input);

    println!("best output:\n");
    display_output(&output.0, output.1);

    Ok(())
}

fn find_most_likely_output(input: &Vec<Spotlight>) -> (Vec<Spotlight>, i32) {
    let mut current = input.clone();
    let mut bounds = spotlight_bounding_rect(&current);
    let mut timer = 0;

    loop {
        let updated = update(&current);
        let new_bounds = spotlight_bounding_rect(&updated);
        if new_bounds.h > bounds.h {
            return (current, timer);
        }

        timer += 1;
        current = updated;
        bounds = new_bounds;
    }
}

fn update(input: &Vec<Spotlight>) -> Vec<Spotlight> {
    let mut new = input.clone();

    for s in new.iter_mut() {
        s.update();
    }

    new
}

fn display_output(spotlights: &Vec<Spotlight>, time: i32) {
    let rect = spotlight_bounding_rect(spotlights);
    let mut strs = vec!(vec!(' '; rect.w as usize + 1); rect.h as usize + 1);

    for p in spotlights.iter().map(|s| &s.position) {
        strs[(p.y - rect.y) as usize][(p.x - rect.x) as usize] = '#';
    }

    let strs = strs.iter()
        .map(|x| x.into_iter().collect::<String>())
        .collect::<Vec<String>>();

    for s in &strs {
        println!("{}", s);
    }

    println!("Would have taken {} seconds", time);
}

fn spotlight_bounding_rect(spotlights: &Vec<Spotlight>) -> Rect {
    bounding_rect(spotlights.iter().map(|s| &s.position))
}

fn bounding_rect<'a>(points: impl Iterator<Item=&'a Vec2>) -> Rect {
    let mut min_x = std::i32::MAX;
    let mut min_y = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut max_y = std::i32::MIN;

    for p in points {
        min_x = std::cmp::min(min_x, p.x);
        min_y = std::cmp::min(min_y, p.y);
        max_x = std::cmp::max(max_x, p.x);
        max_y = std::cmp::max(max_y, p.y);
    }

    Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
}

fn read_input(filename: &String) -> AppResult<Vec<Spotlight>> {
    let input = read_file_input(filename)?;
    parse_input(&input)
}

fn parse_input(input: &String) -> AppResult<Vec<Spotlight>> {
    let mut results = Vec::new();
    for line in input.lines() {
        results.push(Spotlight::parse(&line)?);
    }
    Ok(results)
}

#[derive(Clone)]
struct Spotlight {
    position: Vec2,
    velocity: Vec2,
}

impl Spotlight {
    fn parse(input: &str) -> AppResult<Self> {
        let i = input.split(&['<', '>'][..]).collect::<Vec<&str>>();
        if i.len() != 5 {
            return Err(AppError::AppError(String::from("Invalid input")));
        }

        Ok(Spotlight {
            position: Vec2::parse(i[1])?,
            velocity: Vec2::parse(i[3])?,
        })
    }

    fn update(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }
}

#[derive(Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn parse(input: &str) -> AppResult<Self> {
        let mut x = String::new();
        let mut y = String::new();
        let mut swapped = false;

        let mut current = &mut x;

        for ch in input.chars() {
            if ch == ' ' {
                continue;
            } else if ch == ',' {
                if swapped {
                    return Err(AppError::AppError(String::from("Invalid input Vec2")));
                }
                current = &mut y;
                swapped = true;
                continue;
            }

            current.push(ch);
        }

        Ok(Vec2 {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Rect {
    fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Rect { x, y, w, h }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_example() {
//         unimplemented!();
//     }
// }
