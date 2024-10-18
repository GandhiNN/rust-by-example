struct Person {
    name: String,
}

// we are returning a substring of words, so &str is appropriate
fn first_word(words: &str) -> &str {
    words
        .split_whitespace()
        .next()
        .expect("words should not be empty")
}

fn main() {
    let sentence = "Hello world!";

    println!("{}", first_word(sentence));

    let owned = String::from("A string");

    println!("{}", first_word(&owned));
    println!("{}", first_word(&owned));

    let person = Person {
        name: String::from("Ngakan Gandhi"),
    };

    println!("{}", first_word(person.name.as_str()));
}
