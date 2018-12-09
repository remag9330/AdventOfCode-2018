use util;

pub fn run_part_1(args: &[String]) {
    util::run_part_n("1", args, blah);
}

fn blah(filename: &String) -> util::AppResult {
    let input = read_input(filename)?;

    Ok(())
}

fn read_input(filename: &String) -> util::AppResult {
    let input = util::read_file_input(filename)?;
    Ok(())
}
