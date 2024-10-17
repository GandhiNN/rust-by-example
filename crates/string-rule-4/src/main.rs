// accept a ref to string, make it uppercase
// it cannot be returned as a ref to string
// so return an owned string
fn first_word_uppercase(words: &str) -> String {
    words
        .split_whitespace()
        .next()
        .expect("words should not be empty")
        .to_uppercase()
}

fn main() {
    let sentence = "Hello, world!";

    println!("{}", first_word_uppercase(sentence));

    let owned = String::from("A string");

    println!("{}", first_word_uppercase(&owned));
    println!("{}", first_word_uppercase(&owned));
}