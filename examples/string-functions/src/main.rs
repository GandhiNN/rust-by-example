use std::sync;
use std::rc;

fn print_me(msg: &str) {
    println!("msg = {}", msg);
}

fn main() {
    let string: &str = "hello world";
    print_me(string);

    let owned_string: String = "hello world".to_string();
    print_me(&owned_string);

    let counted_string: rc::Rc<String> = rc::Rc::new("hello world".to_string());
    print_me(&counted_string);

    let atomically_counted_string: sync::Arc<String> = sync::Arc::new("hello world".to_string());
    print_me(&atomically_counted_string);
}