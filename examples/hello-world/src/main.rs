fn main() {
    // Basic print
    println!("{} days", 31);

    // Using the positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Using the format character ':'
    println!("Base 10:          {}", 69420);
    println!("Base 2 (binary): {:b}", 69420);
    println!("Base 8 (octal):  {:o}", 69420);
    println!("Base 16 (hex):   {:x}", 69420);

    // Right-justify text with a specified width
    println!("{number:>5}", number = 1);

    // Pad numbers with extra zeroes
    println!("{number:0>5}", number = 1);
    // left-adjust by flipping the sign
    println!("{number:0<5}", number = 1);

    // Use named arguments in the format specifier
    println!("{number:0>width$}", number = 1, width = 5);

    // Rust checks that the correct number of arguments are used
    println!("My name is {0}, {1} {0}", "Bond", "James")
}
