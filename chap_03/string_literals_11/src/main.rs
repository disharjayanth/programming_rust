fn main() {
    let speech = "\"Ouch!\" said the well.\n";
    print!("{}", speech);

    println!(
        "In the room the women come and go,
    Singing of Mount Abora"
    );

    println!(
        "It was a bright, cold day in April, and \
    there were four of us-\
    more or less."
    );

    let default_win_install_path = r"C:\Program Files\Gorillas";
    println!("default windows install path: {}", default_win_install_path);

    println!(
        r####"
        This raw string standard with 'r####"'.
        Therefore it does not end until we reach a quote mark ('"')
        followed immediately by four pound signs ('####');"####
    );
}
