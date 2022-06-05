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

fn main() {
    println!("Hello, world!");
}
