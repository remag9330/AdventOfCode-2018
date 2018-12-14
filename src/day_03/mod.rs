use crate::util;

const FABRIC_SIZE: usize = 1000;

pub fn run_part_1(args: &[String]) {
    util::run_part_n("3", args, find_overlaps);
}

pub fn run_part_2(args: &[String]) {
    util::run_part_n("3", args, find_standalone);
}

fn find_overlaps(filename: &String) -> util::AppResult {
    let claims = read_claims(filename)?;
    let result = count_overlaps(&claims);

    println!("Overlaps: {}", result);

    Ok(())
}

fn count_overlaps(claims: &Vec<Claim>) -> i32 {
    let mut fabric = [InchState::Unused; FABRIC_SIZE * FABRIC_SIZE];

    for claim in claims.iter() {
        for (x, y) in claim.rect.iter() {
            let new_val = match fabric[(y * FABRIC_SIZE + x)] {
                InchState::Unused => InchState::Used,
                _ => InchState::Overused
            };

            fabric[y * FABRIC_SIZE + x] = new_val;
        }
    }

    count_fabric_overused(&fabric)
}

fn count_fabric_overused(state: &[InchState]) -> i32 {
    let mut total_overused = 0;
    for inch in state.iter() {
        if *inch == InchState::Overused {
            total_overused += 1;
        }
    }

    total_overused
}

fn find_standalone(filename: &String) -> util::AppResult {
    let claims = read_claims(filename)?;

    for claim in claims.iter() {
        if !overlaps_any(claim, claims.iter().filter(|c| c.id != claim.id)) {
            println!("Standalone fabric id: {}", claim.id);
        }
    }

    Ok(())
}

fn overlaps_any<'a>(claim: &Claim, rest: impl Iterator<Item = &'a Claim>) -> bool {
    for compare in rest {
        if claim.rect.overlaps(&compare.rect) {
            return true;
        }
    }

    return false;
}

fn read_claims(filename: &String) -> Result<Vec<Claim>, util::AppError> {
    let contents = util::read_file_input(filename)?;
    let result = parse_claims(&contents);
    Ok(result)
}

fn parse_claims(input: &str) -> Vec<Claim> {
    input.lines().map(Claim::parse).collect()
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum InchState {
    Unused,
    Used,
    Overused,
}

struct Claim {
    id: i32,
    rect: Rectangle,
}

impl Claim {
    fn parse(input: &str) -> Claim {
        let mut parts = input.split(" @ ");
        let id = parts.next().expect("Empty claim text");
        let id = id.chars().skip(1).collect::<String>().parse::<i32>().expect("Could not determine claim ID");

        let rect = Rectangle::parse(parts.next().expect("Could not find rectangle part of claim"));

        Claim {
            id: id,
            rect: rect
        }
    }
}

struct Rectangle {
    top: i32,
    left: i32,
    width: i32,
    height: i32,
}

impl Rectangle {
    fn parse(input: &str) -> Rectangle {
        let mut parts = Vec::new();
        let mut current = String::new();
        for c in input.chars() {
            if c == ' ' {
                continue;
            }

            if c == ',' || c == ':' || c == 'x' {
                parts.push(current.parse::<i32>().expect("Invalid rect definition"));
                current = String::new();
            } else {
                current.push(c);
            }
        }
        parts.push(current.parse::<i32>().expect("Invalid rect definition"));

        if parts.len() != 4 {
            panic!("Invalid rect definition");
        }


        Rectangle { left: parts[0], top: parts[1], width: parts[2], height: parts[3]}
    }

    fn iter(&self) -> RectPointIterator {
        RectPointIterator::new(self)
    }

    fn overlaps(&self, other: &Rectangle) -> bool {
        let self_right = self.left + self.width;
        let other_right = other.left + other.width;
        let self_bottom = self.top + self.height;
        let other_bottom = other.top + other.height;

        self.left < other_right && self_right > other.left &&
            self.top < other_bottom && self_bottom > other.top
    }
}

struct RectPointIterator<'a> {
    position: i32,
    rect: &'a Rectangle,
}

impl<'a> RectPointIterator<'a> {
    fn new(rect: &'a Rectangle) -> RectPointIterator {
        RectPointIterator {
            position: -1,
            rect: rect,
        }
    }
}

impl<'a> Iterator for RectPointIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        self.position += 1;

        if self.position >= self.rect.width * self.rect.height {
            return None
        }

        let x = self.rect.left + (self.position % self.rect.width as i32);
        let y = self.rect.top + (self.position / self.rect.width as i32);

        if y as usize >= FABRIC_SIZE {
            None
        } else {
            Some((x as usize, y as usize))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claim_parsing() {
        let claim = Claim::parse("#1 @ 829,837: 11x22");

        assert_eq!(claim.id, 1);
        assert_eq!(claim.rect.left, 829);
        assert_eq!(claim.rect.top, 837);
        assert_eq!(claim.rect.width, 11);
        assert_eq!(claim.rect.height, 22);
    }

    #[test]
    fn test_count_overlaps() {
        let claims = parse_claims("#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2");

        assert_eq!(count_overlaps(&claims), 4);
    }
}
