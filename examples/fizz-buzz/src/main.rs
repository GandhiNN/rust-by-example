use std::borrow::Cow;

use fizz_buzz::fizz_buzz_enhanced;

fn fizzbuzz_simple() {
    for i in 1..101 {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", i);
        }
    }
}

fn fizzbuzz_cast_to_string() {
    for i in 1..101 {
        let result: String = if i % 15 == 0 {
            String::from("FizzBuzz")
        } else if i % 5 == 0 {
            String::from("Buzz")
        } else if i % 3 == 0 {
            String::from("Fizz")
        } else {
            i.to_string()
        };
        println!("{}", result);
    }
}

// This creates an intermediate String
// everytime the condition is reached
fn fizzbuzz_function_return(i: i32) -> String {
    if i % 15 == 0 {
        "FizzBuzz".to_string()
    } else if i % 5 == 0 {
        "Buzz".to_string()
    } else if i % 3 == 0 {
        "Fizz".to_string()
    } else {
        i.to_string()
    }
}

fn fizzbuzz_cow(i: i32) -> Cow<'static, str> {
    if i % 15 == 0 {
        "FizzBuzz".into()
    } else if i % 5 == 0 {
        "Buzz".into()
    } else if i % 3 == 0 {
        "Fizz".into()
    } else {
        i.to_string().into()
    }
}

fn main() {
    fizzbuzz_simple();
    fizzbuzz_cast_to_string();

    // Try the function
    for i in 1..101 {
        println!("{}", fizzbuzz_function_return(i));
    }

    // Try the cow
    for i in 1..101 {
        println!("{}", fizzbuzz_cow(i));
    }

    // Try the enhanced
    for i in 1..101 {
        println!("{}", fizz_buzz_enhanced(i));
    }

    // Try the enhanced function using the Map
    for f in (1..101).map(fizz_buzz_enhanced) {
        println!("{}", f);
    }
}
