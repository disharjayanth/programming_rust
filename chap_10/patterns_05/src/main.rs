enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

enum RoughTime {
    InThePastTime(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePastTime(units, count) => format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow => format!("just now"),
        RoughTime::InTheFuture(units, count) => format!("{} {} from now", count, units.plural()),
    }
}

fn main() {
    println!("Hello, world!");
}
