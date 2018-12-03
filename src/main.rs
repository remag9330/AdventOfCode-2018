use std::env;

mod day_01;
mod day_02;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1);

    match day {
        Some(day) => {
            run_day(day, &args[2..]);
        },
        _ => {
            println!("Usage: <day number> ...[args]")
        }
    }
}

fn run_day(day: &String, args: &[String]) {
    match day.as_ref() {
        "1.1" => day_01::run_part_1(args),
        "1.2" => day_01::run_part_2(args),
        "2.1" => day_02::run_part_1(args),
        _ => println!("Day not found: {}", day)
    };
}
