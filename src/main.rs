use advent_of_code_2019::common;
use advent_of_code_2019::day1;
use std::env;

fn main() {
    // Grab an iterator on the arguments, skipping the first one (the program name)
    let mut args_iter = env::args().skip(1);

    // Grab the day name which should be the first argument
    let day_name = args_iter.next();
    if let None = day_name {
        common::fail(
            "Please provide at least the name of the day to run, i.e. \"day1\".",
            common::NO_DAY_PROVIDED_EXIT_CODE,
        )
    }

    // Collect the remaining arguments into their own Vector
    let day_args: Vec<String> = args_iter.collect();

    // Resolve the day runner itself
    let day_name = &day_name.unwrap()[..];
    let day_runner: Box<dyn common::DayRunner> = Box::new(match day_name {
        "day1" => day1::Day1Runner {},
        _ => common::fail(
            &format!("Not a valid day: {}", day_name)[..],
            common::BAD_DAY_PROVIDED_EXIT_CODE,
        ),
    });

    // Then simply delegate to the day runner to run
    match (*day_runner).run(&day_args) {
        Err(err) => println!("{}", err),
        _ => (),
    }
}
