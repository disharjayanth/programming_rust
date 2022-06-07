enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "second",
            TimeUnit::Minutes => "minute",
            TimeUnit::Hours => "hour",
            TimeUnit::Days => "day",
            TimeUnit::Months => "month",
            TimeUnit::Years => "year",
        }
    }
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
        RoughTime::InTheFuture(unit, 1) => format!("a {} from now", unit.singular()),
        RoughTime::InTheFuture(units, count) => format!("{} {} from now", count, units.plural()),
    }
}

fn main() {
    println!(
        "{}",
        rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Days, 5))
    );

    println!(
        "{}",
        rough_time_to_english(RoughTime::InThePastTime(TimeUnit::Years, 4))
    );

    println!("{}", rough_time_to_english(RoughTime::JustNow));

    println!(
        "{}",
        rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Months, 1))
    );
}
