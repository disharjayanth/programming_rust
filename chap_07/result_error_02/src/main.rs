use std::io;

fn display_weather(hometown: &String, report: &WeatherReport) {
    println!(
        "{hometown} weather is {} and it's {}",
        report.tmp, report.info
    );
}

#[derive(Debug)]
struct WeatherReport {
    tmp: i32,
    info: String,
}

fn get_weather(location: &String) -> Result<WeatherReport, io::Error> {
    if location == &"".to_string() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "empty location string!",
        ));
    }
    let wr = WeatherReport {
        tmp: -10,
        info: format!("{} is {}", location, "too cold".to_string()),
    };

    Ok(wr)
}

fn main() {
    let hometown = "bangalore".to_string();
    match get_weather(&hometown) {
        Ok(report) => {
            display_weather(&hometown, &report);
        }
        Err(err) => {
            println!("error querying the weather: {}", err);
        }
    };

    match get_weather(&"".to_string()) {
        Ok(report) => {
            display_weather(&hometown, &report);
        }
        Err(err) => {
            println!("error querying the weather: {}", err);
        }
    };
}
