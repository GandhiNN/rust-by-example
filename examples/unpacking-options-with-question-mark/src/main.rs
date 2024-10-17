fn next_birthday(current_age: Option<u8>) -> Option<String> {
    // if current_age is None, returns None
    // if current_age is Some(value<u8>), returns value<u8>
    let next_age: u8 = current_age? + 1;
    Some(format!("Next year I will be {}", next_age))
}

fn next_birthday_v2(current_age: Option<u8>) -> Option<String> {
    let next_age = match current_age {
        Some(val) => val,
        None => return None,
    };
    Some(format!("Next year I will be {}", next_age + 1))
}

fn main() {
    let age1 = Some(10);
    let msg = next_birthday(age1).unwrap();
    println!("{}", msg);

    let age2 = None;
    let msg2 = next_birthday_v2(age2).unwrap(); // panic because unwrap on None value
    println!("{}", msg2);
}