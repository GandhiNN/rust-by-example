// A concret type 'A'
struct A;

// In defining the type `Single`, the first use of `A` is not preceded by `<A>`
// Therefore, `Single` is a concret type, and `A` is defined as above.
struct Single(A);

struct SingleGen<T>(T);

fn main() {
    // `Single` is concrete and explicitly takes `A`
    let _s = Single(A);

    // Create a variable `_char` of type `SingleGen<char>`
    // and give it the value `SingleGen('a')`
    // Here, `SingleGen` has a type parameter explicitly specified.
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen` can also have a type parameter implicitly specified
    let _t = SingleGen(A); // Uses `A` defined at the top.
    let _i32 = SingleGen(6); // Uses `i32`
    let _char = SingleGen('a'); // Uses 'char'
}
