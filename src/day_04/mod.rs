use util;

pub fn run_part_1(args: &[String]) {
    util::run_part_n("1", args, find_most_asleep_guard);
}

fn find_most_asleep_guard(_filename: &String) -> util::AppResult {
    // let input = read_input(filename)?;

    Ok(())
}

// fn read_input(filename: &String) -> util::AppResult {
//     let input = util::read_file_input(filename)?;
//     let parsed = parse(&input.lines());
//     Ok(())
// }

// fn parse(input: &Iterator<Item = &str>) -> i32 {
//     0
// }

// struct Day {
//     guard_id: String,

// }
