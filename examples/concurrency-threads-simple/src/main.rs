use std::sync::mpsc;
use std::thread;
use std::vec::Vec;

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();

    for i in 0..5 {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            let result = i * 2;
            tx.send(result).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    drop(tx); // Close the sending channel

    while let Ok(result) = rx.recv() {
        println!("{}", result);
    }
}
