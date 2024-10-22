use std::error::Error;
use std::io::{self};
use std::process;

mod cli;

fn open_csv_records_from_stdin() -> csv::Reader<io::Stdin> {
    let bufreader = io::stdin();
    csv::Reader::from_reader(bufreader)
}

fn handle_error_using_expect() {
    println!("\nReading and Handling CSV Error using expect!");
    let mut csv = open_csv_records_from_stdin();
    for result in csv.records() {
        let record = result.expect("A CSV record");
        println!("{:?}", record);
    }
}

fn handle_error_using_match() {
    println!("\nReading and Handling CSV Error using Match Statement!");
    let mut csv = open_csv_records_from_stdin();
    for record in csv.records() {
        match record {
            Ok(r) => println!("{:?}", r),
            Err(e) => {
                println!("Error reading CSV from stdin: {}", e);
                process::exit(1);
            }
        }
    }
}

fn handle_error_using_returned_error_trait() -> Result<(), Box<dyn Error>> {
    println!("\nReading and Handling CSV Error using Returned Error Trait!");
    let mut csv = open_csv_records_from_stdin();
    for record in csv.records() {
        match record {
            Ok(r) => println!("{:?}", r),
            Err(e) => return Err(From::from(e)),
        }
    }
    Ok(())
}

fn handle_error_using_question_mark() -> Result<(), Box<dyn Error>> {
    println!("\nReading and Handling CSV Error using Question Mark Operator!");
    let mut csv = open_csv_records_from_stdin();
    for record in csv.records() {
        let r = record?;
        println!("{:?}", r);
    }
    Ok(())
}

fn main() {
    // Runtime argument parsing
    let matches = cli::cmd().get_matches();
    let opt_passed = matches.get_one("option");
    match opt_passed {
        Some(1) => handle_error_using_expect(),
        Some(2) => handle_error_using_match(),
        Some(3) => {
            if let Err(e) = handle_error_using_returned_error_trait() {
                println!("{}", e);
                process::exit(1);
            }
        }
        Some(4) => {
            if let Err(e) = handle_error_using_question_mark() {
                println!("{}", e);
                process::exit(1);
            }
        }
        _ => process::exit(1),
    };
}
