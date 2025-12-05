macro_rules! register_days {
    ($($day_num:expr => $day_mod:ident),* $(,)?) => {
        $(pub mod $day_mod;)*

        pub fn solve_day(day: u32, input: &str) {
            match day {
                $($day_num => $day_mod::solve(input),)*
                _ => {
                    eprintln!("Day {} not implemented yet", day);
                    std::process::exit(1);
                }
            }
        }
    };
}

register_days! {
    0 => day00,
    1 => day01,
    2 => day02,
    3 => day03,
    4 => day04_compatible,
    5 => day05,
}
