// Declaring Struct
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

// Implement the Person struct
impl Person {
    // Builder
    fn new(first_name: String, last_name: String) -> Self {
        Person {
            first_name,
            last_name,
        }
    }
    // Retrieve the full name
    // This method will take ownership of the self
    fn full_name_owned(self) -> String {
        format!("{}, {}", self.first_name, self.last_name)
    }
    // Retrieve the full name
    // This method will borrow from self i.e. the struct will still be usable afterwards
    fn full_name_borrow(&self) -> String {
        format!("{}, {}", self.first_name, self.last_name)
    }
}

fn main() {
    let person_name = Person::new("Ngakan".to_string(), "Gandhi".to_string());
    let full_name_borrow = person_name.full_name_borrow(); // just borrow the person_name struct
    let full_name = person_name.full_name_owned(); // ownership of person_name struct moved here
    println!("My full name is: {}", full_name); // full_name is destroyed here
    println!("My full name (borrowed) is: {}", full_name_borrow);
}
