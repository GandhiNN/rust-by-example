struct Person {
    name: String,
}

fn first_word(words: String) -> String {
    words
        .split_whitespace()
        .next()
        .expect("words should not be empty")
        .to_string()
}

fn main() {
    let sentence = "Hello, world!";

    println!("{}", first_word(sentence.to_string()));

    let owned = String::from("A string");

    // If we don't clone here, we cannot use owned the second time
    println!("{}", first_word(owned.clone()));
    println!("{}", first_word(owned));

    // Construct person
    let person = Person {
        name: String::from("Ngakan Gandhi"),
    };

    println!("{}", person.name);
    println!("{}", first_word(person.name));
}
