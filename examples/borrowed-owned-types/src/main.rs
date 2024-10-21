fn main() {
    let pet_name: String = String::from("Gary"); // Owned
    let pet_owner_name: String = String::from("Karl"); // Owned

    let borrowed_type_variable: &str = "this is a string";
    let owned_type_variable: String = borrowed_type_variable.to_owned(); // Converting string reference to owned type

    println!("borrowed - {}", borrowed_type_variable);
    println!("owned - {}", owned_type_variable);
    println!("{}", pet_name);
    println!("{}", pet_owner_name);
}
