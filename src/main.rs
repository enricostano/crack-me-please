#[macro_use]
extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("crack-me-please")
        .version("0.1")
        .author("Enrico S. <enricostn@gmail.com>")
        .about("Please crack me!")
        .arg(Arg::with_name("digit")
             .short("d")
             .long("digit")
             .value_name("DIGIT")
             .help("A digit to crack")
             .required(true)
             .takes_value(true))
        .get_matches();

    let digit = value_t!(matches, "digit", u32).unwrap();
    println!("Your digit is: {}", digit);

    if digit > 50 {
        println!("Nice try, but you didn't crack me. Try again!");
    } else {
        println!("YOU WIN!");
    }
}
