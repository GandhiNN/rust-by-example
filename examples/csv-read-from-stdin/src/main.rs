use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    // Create a CSV parser that reads data from stdin
    // Example stdin => piped output of `cat uspop.csv`
    let mut reader = csv::Reader::from_reader(io::stdin());

    // Loop over each record
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}