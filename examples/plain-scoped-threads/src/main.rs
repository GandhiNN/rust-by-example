use std::thread;

// fn plain() {
//     let s = String::from("Hello");
//     thread::spawn(|| {
//         println!("Length: {}", s.len());
//     });
// }

fn scoped() {
    let s = String::from("Hello");

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Length: {}", s.len());
        });
    });
}

fn main() {
    scoped();
}
