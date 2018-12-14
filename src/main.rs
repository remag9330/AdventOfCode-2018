use std::env;

mod util;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
// mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;

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
        "2.2" => day_02::run_part_2(args),
        "3.1" => day_03::run_part_1(args),
        "3.2" => day_03::run_part_2(args),
        "4.1" => day_04::run_part_1(args),
        // "4.2" => day_04::run_part_2(args),
        "5.1" => day_05::run_part_1(args),
        "5.2" => day_05::run_part_2(args),
        // "6.2" => day_06::run_part_2(args),
        // "6.2" => day_06::run_part_2(args),
        "7.1" => day_07::run_part_1(args),
        "7.2" => day_07::run_part_2(args),
        "8.1" => day_08::run_part_1(args),
        "8.2" => day_08::run_part_2(args),
        "9.1" => day_09::run_part_1(args),
        "9.2" => day_09::run_part_2(args),
        "10.1" => day_10::run_part_1(args),
        "10.2" => day_10::run_part_2(args),
        _ => println!("Day not found: {}", day)
    };
}
