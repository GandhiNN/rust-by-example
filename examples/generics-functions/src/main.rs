struct A; // Concrete type `A`
struct S(A); // Concrete type `S`
struct SGen<T>(T); // Generic type `SGen`

// The following functions all take ownership of the variable passed into
// them and immediately go out of scope, freeing the variable.

// This is a generic function
fn reg_fn(_s: S) {}

// `A` is not specified as generic, so the function is also not generic
fn gen_spec_t(_s: SGen<A>) {}

// i32 is not a generic type, this function is also not generic
fn gen_spec_i32(_s: SGen<i32>) {}

// This function is generic over `T`
fn generic<T>(_s: SGen<T>) {}

fn main() {
    // Using the non-generic functions
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    // Explicitly specified type parameter `char` to `generic()`
    generic::<char>(SGen('a'));

    // Implicitly specified type parameter `char` to `generic()`
    generic(SGen('c'));
}