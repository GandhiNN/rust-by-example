use clap::{Arg, ArgGroup, Command};

fn main() {
    // Build the CLI by defining the configuration using the builder pattern
    let cmd = Command::new("NumberCalculator")
        .version("1.0")
        .about("Calculates square or cube of a number based on choice of operation")
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .num_args(1)
                .value_parser(clap::value_parser!(i32))
                .required(true),
        )
        .arg(Arg::new("square").short('s').long("square").num_args(0))
        .arg(Arg::new("cube").short('c').long("cube").num_args(0))
        .group(
            ArgGroup::new("operation")
                .args(["square", "cube"])
                .required(true),
        );

    // Runtime argument parsing
    let matches = cmd.get_matches();
    let number_passed = matches.get_one("number");
    let number: i32 = match number_passed {
        Some(x) => *x,
        None => 0,
    };

    if let Some(true) = matches.get_one("cube") {
        println!("Cube of {} is: {}", number, number * number * number);
    } else if let Some(true) = matches.get_one("square") {
        println!("Square of {} is: {}", number, number * number);
    }
}
