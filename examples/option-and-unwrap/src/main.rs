fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Yuck! too sugary"),
        Some(inner) => println!("{}? How nice", inner),
        None => println!("No drink? oh well."),
    }
}

fn drink(drink: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`
    let inside = drink.unwrap();
    if inside == "lemonade" {
        panic!("aaaa!");
    }
    println!("I love {}s!!", inside);
}

fn main() {
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;
    
    drink(coffee);
    drink(nothing);
}