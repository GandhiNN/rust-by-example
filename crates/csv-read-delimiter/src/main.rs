use std::{error::Error, fs, process};

fn run(content: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .comment(Some(b'#'))
        .from_reader(content.as_bytes());

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    let fpath = "strange.csv";
    let contents = fs::read_to_string(fpath).expect("Cannot open the file");
    if let Err(err) = run(&contents) {
        println!("{}", err);
        process::exit(1);
    }
}
