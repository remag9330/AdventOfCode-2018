use crate::util::*;

pub fn run_part_1(args: &[String]) {
    run_part_n("1", args, |_| Ok(()));
}

pub fn run_part_2(args: &[String]) {
    run_part_n("2", args, |_| Ok(()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        unimplemented!();
    }
}
