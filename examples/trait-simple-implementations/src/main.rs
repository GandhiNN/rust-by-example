struct Sparrow {}
struct Plane {}

trait Flyable {
    fn flying(&self);
}

trait Hoppable {
    fn hopping(&self);
}

impl Flyable for Sparrow {
    fn flying(&self) {
        println!("Sparrow flying!");
    }
}

impl Hoppable for Sparrow {
    fn hopping(&self) {
        println!("Sparrow hopping!");
    }
}

impl Flyable for Plane {
    fn flying(&self) {
        println!("Plane flying!");
    }
}

fn main() {
    let sparrow = Sparrow {};
    let plane = Plane {};

    sparrow.flying();
    sparrow.hopping();
    plane.flying();
}
