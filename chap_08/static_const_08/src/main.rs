pub const ROOM_TEMPERATURE: f64 = 20.0;
pub static WEATHER_TEMPERATURE: f64 = 28.0;

// static mut PACKETS_SERVED: usize = 0;

fn main() {
    println!("ROOM_TEMPERATURE is constant: {}", ROOM_TEMPERATURE);
    println!("WEATHER_TEMPERATURE is static: {}", WEATHER_TEMPERATURE);

    // println!(
    //     "PACKET_SERVED is mutable static(not recommended): {}",
    //     PACKETS_SERVED
    // );
}
