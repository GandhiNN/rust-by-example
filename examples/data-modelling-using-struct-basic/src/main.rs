#[derive(Debug)]
enum AnimalType {
    Dog,
    Cat,
    Bird,
}

struct Animal {
    animal_type: AnimalType,
    name: String,
    age: i32,
}

fn print_animal_info(animal: Animal) {
    match animal.animal_type {
        AnimalType::Dog => println!("{} is a {}-year-old dog.", animal.name, animal.age),
        AnimalType::Cat => println!("{} is a {}-year-old cat.", animal.name, animal.age),
        AnimalType::Bird => println!("{} is a {}-year-old bird.", animal.name, animal.age),
    }
}

fn main() {
    let dog = Animal {
        animal_type: AnimalType::Dog,
        name: String::from("Buddy"),
        age: 3,
    };

    let cat = Animal {
        animal_type: AnimalType::Cat,
        name: String::from("Mittens"),
        age: 2,
    };

    let bird = Animal {
        animal_type: AnimalType::Bird,
        name: String::from("Birdie"),
        age: 3,
    };

    print_animal_info(dog);
    print_animal_info(cat);
    print_animal_info(bird);
}
