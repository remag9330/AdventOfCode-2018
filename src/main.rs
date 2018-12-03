use std::env;

mod day_01;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).map(|day| day.parse::<i32>());

    match day {
        Some(Ok(day)) => {
            run_day(day, &args[2..]);
        },
        _ => {
            println!("Usage: <day number> ...[args]")
        }
    }
}

fn run_day(day: i32, args: &[String]) {
    match day {
        1 => day_01::run(args),
        _ => println!("Day not found: {}", day)
    };
}
