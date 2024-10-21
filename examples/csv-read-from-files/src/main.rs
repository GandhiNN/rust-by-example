mod csv_reader;
use std::fs::File;

fn open(fname: &str) -> File {
    std::fs::File::open(fname).expect("Cannot find the file")
}

fn main() {
    csv_reader::read_csv_content_line_by_line();
    csv_reader::read_csv_content_all_at_once();

    // Read from file
    let filename = "hw_25000.csv";
    let csvfile_handler = open(filename);
    csv_reader::read_csv_content_from_file(&csvfile_handler);

    // Parse header from file
    csv_reader::parse_csv_header_from_file(&csvfile_handler);

    // Parse csv from file into struct
    csv_reader::parse_csv_file_into_struct(&csvfile_handler);
}
