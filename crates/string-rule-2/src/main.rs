struct Person {
    name: String,
}

// takes reference to string and return an owned string
fn first_word(words: &str) -> String {
    words
        .split_whitespace()
        .next()
        .expect("words should not be empty")
        .to_string()
}

fn main() {
    let sentence = "Hello, world!";

    println!("{}", first_word(sentence));

    let owned = String::from("A string");

    println!("{}", first_word(&owned));
    println!("{}", first_word(&owned));

    let gandhi = Person {
        name: String::from("Ngakan Gandhi"),
    };

    let gandhi_first_name = first_word(gandhi.name.as_str()); // convert String to &str
    println!("{}", gandhi_first_name);
}