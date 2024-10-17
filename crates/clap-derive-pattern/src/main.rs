//! Clap: Using the Derive pattern
use clap::Parser;

/// Define a struct to represent command line arguments using Clap's derive attribute
/// The struct will have built-in help, version, and author information
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    // The number for which to calculate square and cube
    #[arg(short, long)]
    number: f64,
}

fn main() {
    // Parse the command line arguments using the generated Args struct
    let args = Args::parse();

    // Calculate the square and cube of the provided number
    let square = args.number * args.number;
    let cube = args.number * args.number * args.number;

    // Display the result to the user
    println!("Number: {}", args.number);
    println!("Square of number: {}", square);
    println!("Cube of number: {}", cube);
}
