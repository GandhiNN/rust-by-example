/*
We can implement PartialOrd trait on user-defined type,
so we can use `>`, `<`, `>=`, `<=` operators to compare
two value of that type. And this must also implement `PartialEq`
*/

#[derive(PartialEq, PartialOrd)]
struct MyStruct {
    b: i32,
    a: i32,
}

#[derive(PartialEq)]
struct AnotherStruct {
    c: i32,
    d: i32,
}

impl PartialOrd for AnotherStruct {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.c.cmp(&other.c))
    }
}

fn main() {
    let ms1 = MyStruct { a: 1, b: 2 };
    let ms2 = MyStruct { a: 2, b: 1 };
    println!("{}", ms1 > ms2);

    let as1 = AnotherStruct { c: 3, d: 4 };
    let as2 = AnotherStruct { c: 5, d: 6 };
    println!("{}", as1 > as2);
}
