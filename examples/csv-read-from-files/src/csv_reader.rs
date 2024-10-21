use csv::ReaderBuilder;
use std::fs;
use std::io::BufReader;

// A Person struct, representing a row of CSV file
struct Person {
    name: String,
    age: u8,
}

struct Entry {
    index: i32,
    height: f32,
    weight: f32,
}

pub fn read_csv_content_line_by_line() {
    let csv_content: &str = "Name,Age
    John,25
    Jane,30
    Bob,22";

    // Use BufReader to read data in chunks
    let buf = BufReader::new(csv_content.as_bytes());
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(buf);

    // Iterate record and parse into a Person struct
    for result in rdr.records() {
        let record = result.expect("Error reading CSV record");
        let person = Person {
            name: record[0].trim().to_string(),
            age: record[1].parse::<u8>().expect("error parsing age"), // make sure to parse into u8
        };
        println!("Name: {}, Age: {}", person.name, person.age);
    }
}

pub fn read_csv_content_all_at_once() {
    let csv_content: &str = "Name,Age
    John,25
    Jane,30
    Bob,22";

    // Use BufReader to read data in chunks
    let buf = BufReader::new(csv_content.as_bytes());
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(buf);

    // Parse into Person struct
    let persons: Vec<Person> = rdr
        .records()
        .map(|result| result.expect("error reading CSV record"))
        .map(|record| Person {
            name: record[0].trim().to_string(),
            age: record[1].parse::<u8>().expect("error parsing age"),
        })
        .collect();

    for person in &persons {
        println!("Name: {}, Age: {}", person.name, person.age);
    }
}

pub fn read_csv_content_from_file(f: &fs::File) {
    let bufreader = BufReader::new(f);
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(bufreader);

    for result in reader.records() {
        let record = result.expect("error reading CSV record");
        let entry = Entry {
            index: record[0]
                .trim()
                .parse::<i32>()
                .expect("Failed to parse index"),
            height: record[1]
                .trim()
                .parse::<f32>()
                .expect("Failed to parse height"),
            weight: record[2]
                .trim()
                .parse::<f32>()
                .expect("Failed to parse weight"),
        };
        println!(
            "Index: {}, Height: {}, Weight: {}",
            entry.index, entry.height, entry.weight
        );
    }
}

pub fn parse_csv_file_into_struct(f: &fs::File) {
    let bufreader = BufReader::new(f);
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(bufreader);

    for result in csv_reader.records() {
        let record = result.expect("error reading CSV record");
        let entry = Entry {
            index: record[0]
                .trim()
                .parse::<i32>()
                .expect("Failed to parse index"),
            height: record[1]
                .trim()
                .parse::<f32>()
                .expect("Failed to parse height"),
            weight: record[2]
                .trim()
                .parse::<f32>()
                .expect("Failed to parse weight"),
        };
        println!(
            "Index: {}, Height: {}, Weight: {}",
            entry.index, entry.height, entry.weight
        );
    }
}

pub fn parse_csv_header_from_file(f: &fs::File) {
    let buf = BufReader::new(f);
    let mut csvreader = ReaderBuilder::new().has_headers(true).from_reader(buf);

    let header = csvreader.headers().expect("Error parsing header");
    println!("{:?}", header);
}
